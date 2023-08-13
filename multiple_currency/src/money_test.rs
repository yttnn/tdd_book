#[derive(Debug, PartialEq)]
struct Money {
  amount: u32,
  currency: &'static str
}

impl Money {
  fn times(&self, multiplier: u32) -> Money {
    // Money { amount: self.amount * multiplier }
    Money {
      amount: self.amount * multiplier,
      currency: self.currency
    }
  }
  fn equals(&self, obj: Money) -> bool {
    self.amount == obj.amount && self.currency == obj.currency
  }
  fn dollar(amount: u32) -> Money {
    Money {
      amount: amount,
      currency: "USD"
    }
  }
  fn franc(amount: u32) -> Money {
    Money {
      amount: amount,
      currency: "CHF"
    }
  }
  fn currency(&self) -> &str {
    self.currency
  }
}

trait MoneyUtil {
  // fn new(amount: u32) -> Money;
}


#[derive(Debug, PartialEq)]
struct Dollar {
}


#[derive(Debug, PartialEq)]
struct Franc {
}


#[cfg(test)]
mod  tests {
  use crate::money_test::{Money};
  #[test]
  fn test_multiplication() {
    let five = Money::dollar(5);
    assert_eq!(Money::dollar(10), five.times(2));
    assert_eq!(Money::dollar(15), five.times(3));
  }

  #[test]
  fn test_equality() {
    assert!(Money::dollar(5).equals(Money::dollar(5)));
    assert!(!Money::dollar(5).equals(Money::dollar(6)));
    assert!(Money::franc(5).equals(Money::franc(5)));
    assert!(!Money::franc(5).equals(Money::franc(6)));
    assert!(!Money::dollar(5).equals(Money::franc(5)));
  }

  #[test]
  fn test_franc_multiplication() {
    let five = Money::franc(5);
    assert_eq!(Money::franc(10), five.times(2));
    assert_eq!(Money::franc(15), five.times(3));
  }

  #[test]
  fn test_currency() {
    assert_eq!("USD", Money::dollar(1).currency());
    assert_eq!("CHF", Money::franc(1).currency());
  }
}