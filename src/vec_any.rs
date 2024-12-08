use core::any::Any;

pub trait AsAny {
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

pub struct Cat {}
pub struct Dog {
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

pub trait AnyAnimal: Animal + AsAny {}
impl<T: Animal + AsAny> AnyAnimal for T {}
pub type BoxedAnimal = Box<dyn AnyAnimal>;

#[cfg(test)]
mod tests {
  use super::*;

  /// Demonstrates creating a vector of common Animal trait objects,
  /// then downcasting to a concrete Dog type instance to be able to access its fields.
  #[test]
  fn test_downcast_concrete_type_instance_from_trait_object() {
    let c = Cat {};
    let d = Dog { name: "Fido".to_string() };

    let the_zoo = [Box::new(c) as BoxedAnimal, Box::new(d) as BoxedAnimal];

    the_zoo.iter().for_each(|a| a.talk());

    let x = &the_zoo[1];
    let dog = x.as_any().downcast_ref::<Dog>().expect("Failed to downcast to Dog");
  }
}
