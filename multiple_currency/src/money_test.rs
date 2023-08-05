#[derive(Debug)]
pub struct Dollar {
  amount: u32
}

impl Dollar {
  pub fn new(amount: u32) -> Dollar {
    Dollar { amount: amount }
  }
  pub fn times(&mut self, multiplier: u32) {
    self.amount = self.amount * multiplier;
  }
}

#[cfg(test)]
mod  tests {
  use crate::money_test::Dollar;
  #[test]
  fn test_multiplication() {
    let mut five = Dollar::new(5);
    five.times(2);
    assert_eq!(10, five.amount);
  }
}