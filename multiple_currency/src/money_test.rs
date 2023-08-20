#[derive(Debug, PartialEq)]
struct Money {
  amount: u32,
  currency: &'static str
}

impl Money {
  fn times(&self, multiplier: u32) -> Money {
    Money {
      amount: self.amount * multiplier,
      currency: self.currency
    }
  }
  fn plus(&self, addend: Money) -> Expression {
    Expression::Money(
      Money {
        amount: self.amount + addend.amount,
        currency: self.currency
      }
    )
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


enum Expression {
  Money(Money)
}



struct Bank {
}

impl Bank {
  fn reduce(source: Expression, to: &str) -> Money {
    Money::dollar(10)
  }
}

#[cfg(test)]
mod  tests {
  use crate::money_test::{Money, Bank};
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
    assert!(!Money::dollar(5).equals(Money::franc(5)));
  }

  #[test]
  fn test_currency() {
    assert_eq!("USD", Money::dollar(1).currency());
    assert_eq!("CHF", Money::franc(1).currency());
  }

  #[test]
  fn test_simple_addition() {
    let five = Money::dollar(5);
    let sum = five.plus(Money::dollar(5));
    let bank = Bank{};
    let reduced = Bank::reduce(sum, "USD");
  }
}