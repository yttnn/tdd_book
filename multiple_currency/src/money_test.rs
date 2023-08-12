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
  fn dollar(amount: u32) -> Money {
    Money { amount: amount }
  }
  fn franc(amount: u32) -> Money {
    Money { amount: amount }
  }
}

// 一応残しとく...
// trait MoneyUtil {
//   fn new(amount: u32) -> Money;
// }


#[derive(Debug, PartialEq)]
struct Dollar {
}

// impl MoneyUtil for Dollar {
//   fn new(amount: u32) -> Money {
//     Money { amount: amount }
//   }
// }

#[derive(Debug, PartialEq)]
struct Franc {
}

// impl MoneyUtil for Franc {
//   fn new(amount: u32) -> Money {
//     Money { amount: amount }
//   }
// }

#[cfg(test)]
mod  tests {
  use crate::money_test::{Dollar, Franc, Money};
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
    // assert!(!Franc::new(5).equals(Dollar::new(5)));
  }

  #[test]
  fn test_franc_multiplication() {
    let five = Money::franc(5);
    assert_eq!(Money::franc(10), five.times(2));
    assert_eq!(Money::franc(15), five.times(3));
  }
}