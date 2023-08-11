#[derive(Debug, PartialEq)]
struct Dollar {
  amount: u32
}

impl Dollar {
  pub fn new(amount: u32) -> Dollar {
    Dollar { amount: amount }
  }
  pub fn times(&self, multiplier: u32) -> Dollar {
    Dollar { amount: self.amount * multiplier }
  }
  pub fn equals(&self, obj: Dollar) -> bool {
    self.amount == obj.amount
  }
}

#[cfg(test)]
mod  tests {
  use crate::money_test::Dollar;
  #[test]
  fn test_multiplication() {
    let five = Dollar::new(5);
    assert_eq!(Dollar::new(10), five.times(2));
    assert_eq!(Dollar::new(15), five.times(3));
  }

  #[test]
  fn test_equality() {
    assert!(Dollar::new(5).equals(Dollar::new(5)));
    assert!(!Dollar::new(5).equals(Dollar::new(6)));
  }
}