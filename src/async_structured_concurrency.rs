use futures::Stream;
use futures::{pin_mut, StreamExt};
use std::error::Error;
use std::sync::RwLock;

/// Minimal executable example of structured concurrency in Rust as eloquently described in this [blog
/// post](https://emschwartz.me/async-rust-can-be-a-pleasure-to-work-with-without-send-sync-static)
/// of Evan Schwartz.
/// This example proceses 10 incoming demo requests. Each incoming webrequest is processed in a
/// separate scope from the moro crate, similar to an std::thread::scope.
/// With this approach, we don't need types that are Send like Arc to share the database and
/// service dependencies, and no async move is needed. This greatly improves the developer UX.
/// The test_demo test is configured to use the Tokio current thread runtime to emulate thread-per-core.
/// The spawned tasks within the inner moro scope are executed within the same thread, these tasks are
/// not moved between threads and therefore the called future doees not need to be Send.
pub async fn structured_concurrency_demo(incoming: impl Stream<Item = Request>) -> Result<(), Box<dyn Error>> {
  let context = Context::default();
  pin_mut!(incoming);

  moro_local::async_scope!(|scope| {
    while let Some(request) = incoming.next().await {
      let _response: Result<Response, ()> = scope
        .spawn(async {
          let request = request;
          moro_local::async_scope!(|scope| {
            let Ok(two_things) = context.db.load_two_things().await else {
              return Err(());
            };
            for _ in two_things {
              // these two tasks are executed within the scope and are awaited at the end of the
              // inner scope
              scope.spawn(context.service_a.do_something(&request, &context.requests_processed));
            }
            let result_b = context.service_b.do_something(&request, &context.requests_processed);
            let result_c = context.service_c.do_something(&request, &context.requests_processed);
            let (b, c) = futures::try_join!(result_b, result_c).map_err(|_| ())?;
            Ok::<_, ()>(Response { b, c })
          })
          .await
        })
        .await;
    }
  })
  .await;

  println!("do_something executed {} times", *context.requests_processed.read().unwrap());

  Ok(())
}

#[cfg(test)]
mod tests {
  pub use super::*;

  /// View the test output with `just test-watch-all test_demo --success-output immediate`
  #[tokio::test(flavor = "current_thread")]
  async fn test_demo() -> Result<(), Box<dyn Error>> {
    let incoming = futures::stream::iter((1..=10).map(|_| Request));
    structured_concurrency_demo(incoming).await
  }
}

#[derive(Default)]
pub struct Thing;
#[derive(Default)]
pub struct Request;
#[derive(Default)]
pub struct Database;
#[derive(Default)]
#[allow(dead_code)]
pub struct Response {
  b: ServiceResult,
  c: ServiceResult,
}

impl Database {
  async fn load_two_things(&self) -> Result<Vec<Thing>, Box<dyn Error>> {
    Ok((1..=2).map(|_| Thing).collect())
  }
}

#[derive(Default)]
pub struct Service;
impl Service {
  /// simulate a task that takes 200ms
  async fn do_something(&self, _request: &Request, requests_processed: &RwLock<usize>) -> Result<ServiceResult, Box<dyn Error>> {
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let mut count = requests_processed.write().unwrap();
    *count += 1;

    Ok(ServiceResult)
  }
}
#[derive(Default)]
pub struct ServiceResult;

#[derive(Default)]
struct Context {
  db: Database,
  service_a: Service,
  service_b: Service,
  service_c: Service,
  requests_processed: RwLock<usize>,
}