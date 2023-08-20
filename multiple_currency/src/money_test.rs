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
    Expression::Sum(
      Sum { augend: Money { amount: self.amount, currency: self.currency }, addend: addend }
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
  fn reduce(&self, to: &'static str) -> Money {
    Money { amount: self.amount, currency: to }
  }
}


enum Expression {
  Money(Money),
  Sum(Sum)
}

impl Expression {
  fn reduce(&self, to: &'static str) -> Money {
    match self {
      Expression::Money(money) => {
        money.reduce(to)
      },
      Expression::Sum(sum) => {
        sum.reduce(to)
      }
    }
  }
}

struct Sum {
  augend: Money,
  addend: Money
}

impl Sum {
  fn new(augend: Money, addend: Money) -> Self {
    Self { augend: augend, addend: addend }
  }
  fn reduce(&self, to: &'static str) -> Money {
    let amount = self.augend.amount + self.addend.amount;
    Money { amount: amount, currency: to }
  }
}

struct Bank {
}

impl Bank {
  fn new() -> Self {
    Self {}
  }
  fn reduce(&self, source: Expression, to: &'static str) -> Money {
    source.reduce(to)
  }
}

#[cfg(test)]
mod  tests {
  use std::result;

use crate::money_test::{Money, Bank};

use super::{Expression, Sum};
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
    let bank = Bank::new();
    let reduced = bank.reduce(sum, "USD");
    assert_eq!(Money::dollar(10), reduced);
  }

  #[test]
  fn test_plus_return_sum() {
    let five = Money::dollar(5);
    let result = five.plus(Money::dollar(5));
    match result {
      Expression::Money(_) => unreachable!(),
      Expression::Sum(sum) => {
        assert_eq!(five, sum.augend);
        assert_eq!(five, sum.addend);
      }
    }
  }

  #[test]
  fn test_reduce_sum() {
    let sum = Expression::Sum(Sum::new(Money::dollar(3), Money::dollar(4)));
    let bank = Bank::new();
    let result = bank.reduce(sum, "USD");
    assert_eq!(Money::dollar(7), result);
  }

  #[test]
  fn test_reduce_money() {
    let bank = Bank::new();
    let result = bank.reduce(Expression::Money(Money::dollar(1)), "USD");
    assert_eq!(Money::dollar(1), result);
  }
}