use core::any::Any;

trait AsAny {
  fn as_any(&self) -> &dyn Any;
}
impl<T: Any + Animal> AsAny for T {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

pub trait Animal {
  fn talk(&self);
}

struct Cat {}
struct Dog {
  pub name: String,
}

impl Animal for Cat {
  fn talk(&self) {
    println!("Meow!");
  }
}
impl Animal for Dog {
  fn talk(&self) {
    println!("Woof!");
  }
}

trait AnyAnimal: Animal + AsAny {}
impl<T: Animal + AsAny> AnyAnimal for T {}
type BoxedAnimal = Box<dyn AnyAnimal>;

/// Demonstrates creating a vector of common Animal trait objects,
/// then downcasting to a concrete Dog type instance to be able to access its fields.
/// # Panics
///
pub fn downcast_concrete_type_instance_from_trait_object() {
  let c = Cat {};
  let d = Dog { name: "Fido".to_string() };

  let the_zoo = [Box::new(c) as BoxedAnimal, Box::new(d) as BoxedAnimal];

  the_zoo.iter().for_each(|a| a.talk());

  let x = &the_zoo[1];
  let dog = x.as_any().downcast_ref::<Dog>().unwrap();
  assert!(dog.name == "Fido");
}

mod tests {
  #[test]
  fn test_cats_and_dogs() {
    super::downcast_concrete_type_instance_from_trait_object();
  }
}
