use std::{num::ParseFloatError, str::FromStr};

pub struct Angle {
  pub radians: f64,
}

// Never implement From<&str> or From<String> as FromStr is made for this
impl FromStr for Angle {
  type Err = ParseFloatError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let radians = s.parse::<f64>()?;
    Ok(Self { radians })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_from_str() {
    assert_eq!("1.57".parse::<Angle>().expect("Could not parse angle").radians, 1.57)
  }
}
