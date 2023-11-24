use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Money {
    amount: u32,
    currency: &'static str,
}

impl Money {
    fn new(amount: u32, currency: &'static str) -> Self {
        Money { amount, currency }
    }
    fn dollar(amount: u32) -> Expression {
        Expression::Money(Money::new(amount, "USD"))
    }
    fn franc(amount: u32) -> Expression {
        Expression::Money(Money::new(amount, "CHF"))
    }
    fn times(&self, multiplier: u32) -> Expression {
        Expression::Money(Money::new(self.amount * multiplier, self.currency))
    }
    fn plus(&self, addend: Expression) -> Expression {
        Expression::Sum(Sum::new(Expression::Money(self.clone()), addend))
    }
    fn reduce(&self, bank: &Bank, to: &'static str) -> Self {
        let rate = bank.rate(self.currency, to);
        Money::new(self.amount / rate, to)
    }
    fn currency(&self) -> &str {
        self.currency
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Expression {
    Sum(Sum),
    Money(Money),
}

impl Expression {
    fn times(&self, multiplier: u32) -> Expression {
        match self {
            Expression::Sum(sum) => sum.times(multiplier),
            Expression::Money(money) => money.times(multiplier),
        }
    }
    fn plus(&self, addend: Expression) -> Expression {
        match self {
            Expression::Sum(sum) => sum.plus(addend),
            Expression::Money(money) => money.plus(addend),
        }
    }
    fn reduce(&self, bank: &Bank, to: &'static str) -> Money {
        match self {
            Expression::Sum(sum) => sum.reduce(bank, to),
            Expression::Money(money) => money.reduce(bank, to),
        }
    }
    fn currency(&self) -> &str {
        match self {
            Expression::Sum(_) => todo!(),
            Expression::Money(money) => money.currency(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Sum {
    augend: Box<Expression>,
    addend: Box<Expression>,
}

impl Sum {
    fn new(augend: Expression, addend: Expression) -> Self {
        Sum {
            augend: Box::new(augend),
            addend: Box::new(addend),
        }
    }
    fn reduce(&self, bank: &Bank, to: &'static str) -> Money {
        let amount = self.augend.reduce(bank, to).amount + self.addend.reduce(bank, to).amount;
        Money::new(amount, to)
    }
    fn times(&self, multiplier: u32) -> Expression {
        Expression::Sum(Sum::new(
            self.augend.times(multiplier),
            self.addend.times(multiplier),
        ))
    }
    fn plus(&self, addend: Expression) -> Expression {
        Expression::Sum(Sum::new(Expression::Sum(self.clone()), addend))
    }
}

struct Bank {
    rates: HashMap<Pair, u32>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            rates: HashMap::new(),
        }
    }
    fn reduce(&self, source: Expression, to: &'static str) -> Money {
        source.reduce(self, to)
    }
    fn add_rate(&mut self, from: &'static str, to: &'static str, rate: u32) {
        self.rates.insert(Pair::new(from, to), rate);
    }
    fn rate(&self, from: &'static str, to: &'static str) -> u32 {
        if from == to {
            return 1;
        };

        match self.rates.get(&Pair::new(from, to)) {
            Some(rate) => *rate,
            None => panic!("Rate not added"),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Pair {
    from: &'static str,
    to: &'static str,
}

impl Pair {
    fn new(from: &'static str, to: &'static str) -> Self {
        Pair { from, to }
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
        assert_eq!(Money::dollar(10), Expression::Money(reduced));
    }

    #[test]
    fn test_plus_returns_sum() {
        let five = Money::dollar(5);
        let result = five.plus(Money::dollar(5));
        let sum = match result {
            Expression::Sum(sum) => sum,
            _ => panic!("Not Sum"),
        };
        assert_eq!(Box::new(five.clone()), sum.augend);
        assert_eq!(Box::new(five.clone()), sum.addend);
    }

    #[test]
    fn test_reduce_sum() {
        let sum = Expression::Sum(Sum::new(Money::dollar(3), Money::dollar(4)));
        let bank = Bank::new();
        let result = bank.reduce(sum, "USD");
        assert_eq!(Money::dollar(7), Expression::Money(result));
    }

    #[test]
    fn test_reduce_money() {
        let bank = Bank::new();
        let result = bank.reduce(Money::dollar(1), "USD");
        assert_eq!(Money::dollar(1), Expression::Money(result));
    }

    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate("CHF", "USD", 2);
        let result = bank.reduce(Money::franc(2), "USD");
        assert_eq!(Money::dollar(1), Expression::Money(result));
    }

    #[test]
    fn test_identity_rate() {
        assert_eq!(1, Bank::new().rate("USD", "USD"))
    }

    #[test]
    fn test_mixed_addition() {
        let five_bucks = Money::dollar(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();
        bank.add_rate("CHF", "USD", 2);
        let result = bank.reduce(five_bucks.plus(ten_francs), "USD");
        assert_eq!(Money::dollar(10), Expression::Money(result));
    }

    #[test]
    fn test_sum_plus_money() {
        let five_bucks = Money::dollar(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();
        bank.add_rate("CHF", "USD", 2);
        let sum =
            Expression::Sum(Sum::new(five_bucks, ten_francs.clone())).plus(ten_francs.clone());
        let result = bank.reduce(sum, "USD");
        assert_eq!(Money::dollar(15), Expression::Money(result));
    }

    #[test]
    fn test_sum_times() {
        let five_bucks = Money::dollar(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();
        bank.add_rate("CHF", "USD", 2);
        let sum = Expression::Sum(Sum::new(five_bucks, ten_francs)).times(2);
        let result = bank.reduce(sum, "USD");
        assert_eq!(Money::dollar(20), Expression::Money(result));
    }
}
