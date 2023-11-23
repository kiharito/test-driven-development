#[derive(Debug, PartialEq, Clone)]
struct Money {
    amount: u32,
    currency: &'static str,
}

impl Money {
    fn new(amount: u32, currency: &'static str) -> Self {
        Money { amount, currency }
    }
    fn dollar(amount: u32) -> Self {
        Money::new(amount, "USD")
    }
    fn franc(amount: u32) -> Self {
        Money::new(amount, "CHF")
    }
    fn times(&self, multiplier: u32) -> Self {
        Money::new(self.amount * multiplier, self.currency)
    }
    fn plus(&self, addend: Money) -> Expression {
        Expression::Sum(Sum::new(self.clone(), addend))
    }
    fn currency(&self) -> &str {
        self.currency
    }
}

enum Expression {
    Sum(Sum),
}

struct Bank {}

impl Bank {
    fn new() -> Self {
        Bank {}
    }
    fn reduce(&self, source: Expression, to: &'static str) -> Money {
        match source {
            Expression::Sum(sum) => sum.reduce(to),
        }
    }
}

struct Sum {
    augend: Money,
    addend: Money,
}

impl Sum {
    fn new(augend: Money, addend: Money) -> Self {
        Sum { augend, addend }
    }
    fn reduce(&self, to: &'static str) -> Money {
        let amount = self.augend.amount + self.addend.amount;
        Money::new(amount, to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Money::dollar(5).eq(&Money::dollar(5)));
        assert!(!Money::dollar(5).eq(&Money::dollar(6)));
        assert!(!Money::franc(5).eq(&Money::dollar(5)));
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

    #[test]
    fn test_simple_addition() {
        let five = Money::dollar(5);
        let sum = five.plus(Money::dollar(5));
        let bank = Bank::new();
        let reduced = bank.reduce(sum, "USD");
        assert_eq!(Money::dollar(10), reduced);
    }

    #[test]
    fn test_plus_returns_sum() {
        let five = Money::dollar(5);
        let result = five.plus(Money::dollar(5));
        let sum = match result {
            Expression::Sum(sum) => sum,
        };
        assert_eq!(five, sum.augend);
        assert_eq!(five, sum.addend);
    }

    #[test]
    fn test_reduce_sum() {
        let sum = Expression::Sum(Sum::new(Money::dollar(3), Money::dollar(4)));
        let bank = Bank::new();
        let result = bank.reduce(sum, "USD");
        assert_eq!(Money::dollar(7), result);
    }
}
