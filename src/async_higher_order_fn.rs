#![allow(dead_code, unused)]

use std::{error::Error, future::Future, pin::Pin};

/// function that accepts an async function handler
fn higher_order_function(f: FunctionHandler, data: Data) -> Pin<Box<dyn Future<Output = Response>>> {
  f(data)
}

pub type FunctionHandler = &'static (dyn Fn(Data) -> Pin<Box<dyn Future<Output = Response>>>);
pub type Response = Result<ResponseData, Box<dyn Error>>;

fn my_function_handler(data: Data) -> Pin<Box<dyn Future<Output = Response>>> {
  Box::pin(async move { async_dummy().await })
}

/// Dummy async function to demonstrate calling a async function from a async higher order function
async fn async_dummy() -> Result<ResponseData, Box<dyn Error>> {
  Ok(ResponseData { processed: true })
}

pub struct Data {
  title: String,
}

pub struct ResponseData {
  processed: bool,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_call_it() {
    let response = higher_order_function(
      &my_function_handler,
      Data {
        title: "Hello".to_string(),
      },
    )
    .await;
    assert!(response.is_ok());
    assert!(response.unwrap().processed);
  }
}
