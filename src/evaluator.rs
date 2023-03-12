use crate::types::Binop::*;
use crate::types::Expr;
use crate::types::Expr::*;

pub(crate) fn evaluate(expr: Expr) -> f32 {
    match expr {
        ENum(num) => num,
        EBinop(left, op, right) => {
            let left = evaluate(*left);
            let right = evaluate(*right);
            match op {
                EAdd => left + right,
                ESub => left - right,
                EMul => left * right,
                EDiv => left / right,
                EExp => left.powf(right),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::evaluate;
    use crate::types::Binop::*;
    use crate::types::Expr::*;

    #[test]
    fn evaluate_enum_test() {
        let expr = ENum(1234.0);
        assert_eq!(evaluate(expr), 1234.0);
    }

    #[test]
    fn evaluate_eadd_test() {
        let expr = EBinop(Box::new(ENum(12.0)), EAdd, Box::new(ENum(34.0)));
        assert_eq!(evaluate(expr), 46.0);
    }

    #[test]
    fn evaluate_easub_test() {
        let expr = EBinop(Box::new(ENum(12.0)), ESub, Box::new(ENum(34.0)));
        assert_eq!(evaluate(expr), -22.0);
    }

    #[test]
    fn test_evaluate_nested_arithmetic_expression() {
        let expr = EBinop(
            Box::new(EBinop(Box::new(ENum(1.0)), EMul, Box::new(ENum(2.0)))),
            EAdd,
            Box::new(EBinop(
                Box::new(EBinop(Box::new(ENum(6.0)), EExp, Box::new(ENum(2.0)))),
                EDiv,
                Box::new(ENum(5.0)),
            )),
        );
        assert_eq!(evaluate(expr), 9.2);
    }
}
