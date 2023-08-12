#[derive(Debug, PartialEq)]
struct Money {
  amount: u32
}

impl Money {
  fn times(&self, multiplier: u32) -> Money {
    Money { amount: self.amount * multiplier }
  }
  fn equals(&self, obj: Money) -> bool {
    self.amount == obj.amount
  }
}

trait MoneyUtil {
  fn new(amount: u32) -> Money;
}


#[derive(Debug, PartialEq)]
struct Dollar {
}

impl MoneyUtil for Dollar {
  fn new(amount: u32) -> Money {
    Money { amount: amount }
  }
}

#[derive(Debug, PartialEq)]
struct Franc {
  amount: u32
}

impl MoneyUtil for Franc {
  fn new(amount: u32) -> Money {
    Money { amount: amount }
  }
}

#[cfg(test)]
mod  tests {
  use crate::money_test::{Dollar, Franc, MoneyUtil};
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

  #[test]
  fn test_franc_multiplication() {
    let five = Franc::new(5);
    assert_eq!(Franc::new(10), five.times(2));
    assert_eq!(Franc::new(15), five.times(3));
  }
}