#[cfg(test)]
mod tests {
    use crate::tyarith::{typing, Type};
    use crate::tyarith::Term::*;

    #[test]
    fn eval_test() {
        assert_eq!(typing(Zero), Type::Nat);
        assert_eq!(typing(True), Type::Bool);
        assert_eq!(typing(False), Type::Bool);
        assert_eq!(typing(IsZero(Box::new(Zero))), Type::Bool);
        assert_eq!(typing(Pred(Box::new(Zero))), Type::Nat);
        assert_eq!(typing(Succ(Box::new(Zero))), Type::Nat);
        assert_eq!(typing(Pred(Box::new(Succ(Box::new(Zero))))), Type::Nat);
        assert_eq!(typing(If(Box::new(True), Box::new(Pred(Box::new(Zero))), Option::from(Box::new(Succ(Box::new(Pred(Box::new(Zero)))))))), Type::Nat);
        assert_eq!(typing(If(Box::new(False), Box::new(Pred(Box::new(Zero))), Option::from(Box::new(Succ(Box::new(Pred(Box::new(Zero)))))))), Type::Nat);

    }


}

pub mod parser;
pub mod tyarith;
