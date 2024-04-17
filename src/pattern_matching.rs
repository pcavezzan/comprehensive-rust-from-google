/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op { op, left, right } => {
            let left_result = eval(*left);
            match left_result {
                Ok(value) => {
                    let right_result = eval(*right);
                    if right_result.is_err() {
                        return right_result;
                    }
                    return match op {
                        Operation::Add => Ok(value + right_result.unwrap()),
                        Operation::Sub => Ok(value - right_result.unwrap()),
                        Operation::Mul => Ok(value * right_result.unwrap()),
                        Operation::Div => {
                            let divisor = right_result.unwrap();
                            if divisor == 0 {
                                return Err(String::from("division by zero"));
                            }
                            Ok(value / divisor)
                        }
                    };
                }
                Err(_) => left_result
            }
        }
        Expression::Value(value) => Ok(value)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(eval(Expression::Value(19)), Ok(19));
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(10)),
                right: Box::new(Expression::Value(20)),
            }),
            Ok(30)
        );
    }

    #[test]
    fn test_recursion() {
        let term1 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(9)),
        };
        let term2 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(3)),
                right: Box::new(Expression::Value(4)),
            }),
            right: Box::new(Expression::Value(5)),
        };
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(term1),
                right: Box::new(term2),
            }),
            Ok(85)
        );
    }

    #[test]
    fn test_error() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Div,
                left: Box::new(Expression::Value(99)),
                right: Box::new(Expression::Value(0)),
            }),
            Err(String::from("division by zero"))
        );
    }
}