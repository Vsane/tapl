#[cfg(test)]
mod tests {
    use crate::arith::{eval, Term, big_eval};
    use crate::arith::Term::{Zero, True, False, IsZero, Pred, Succ, If};

    #[test]
    fn eval_test() {
        assert_eq!(eval(Zero), Zero);
        assert_eq!(eval(True), True);
        assert_eq!(eval(False), False);
        assert_eq!(eval(IsZero(Box::new(Zero))), True);
        assert_eq!(eval(Pred(Box::new(Zero))), Zero);
        assert_eq!(eval(Succ(Box::new(Zero))), Succ(Box::new(Zero)));
        assert_eq!(eval(Pred(Box::new(Succ(Box::new(Zero))))), Zero);
        assert_eq!(eval(If(Box::new(True), Box::new(Pred(Box::new(Zero))), Option::from(Box::new(Succ(Box::new(Pred(Box::new(Zero)))))))), Zero);
        assert_eq!(eval(If(Box::new(False), Box::new(Pred(Box::new(Zero))), Option::from(Box::new(Succ(Box::new(Pred(Box::new(Zero)))))))), Succ(Box::new(Zero)));

    }

    #[test]
    fn big_eval_test() {
        assert_eq!(big_eval(Zero), Zero);
        assert_eq!(big_eval(True), True);
        assert_eq!(big_eval(False), False);
        assert_eq!(big_eval(IsZero(Box::new(Zero))), True);
        assert_eq!(big_eval(Pred(Box::new(Zero))), Zero);
        assert_eq!(big_eval(Succ(Box::new(Zero))), Succ(Box::new(Zero)));
        assert_eq!(big_eval(Pred(Box::new(Succ(Box::new(Zero))))), Zero);
        assert_eq!(big_eval(If(Box::new(True), Box::new(Pred(Box::new(Zero))), Option::from(Box::new(Succ(Box::new(Pred(Box::new(Zero)))))))), Zero);
        assert_eq!(big_eval(If(Box::new(False), Box::new(Pred(Box::new(Zero))), Option::from(Box::new(Succ(Box::new(Pred(Box::new(Zero)))))))), Succ(Box::new(Zero)));

    }
}

pub mod parser;
pub mod arith;
