#[derive(Debug, Clone, PartialEq, Copy)]
enum Var {
    X, 
    Y, 
    Z,
}

impl Var {
    fn to_string(&self) -> String {
        match self {
            Var::X => "X".to_string(),
            Var::Y => "Y".to_string(),
            Var::Z => "Z".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
enum Const {
    Numeric(i64),
    Named(String),
}

impl Const {
    fn to_string(&self) -> String {
        match self {
            Const::Numeric(n) => n.to_string(),
            Const::Named(s) => s.clone(),
        }
    }
}

#[derive(Clone, Debug)]
enum E {
    Add(Box<E>, Box<E>),
    Neg(Box<E>),
    Mul(Box<E>, Box<E>),
    Inv(Box<E>),
    Const(Const),
    Func { name: String, arg: Box<E> },
    Var(Var),
}

impl E {
    fn add(e1: Box<Self>, e2: Box<Self>) -> Box<Self> {
        Box::new(Self::Add(e1, e2))
    }

    fn neg(e: Box<Self>) -> Box<Self> {
        Box::new(Self::Neg(e))
    }

    fn mul(e1: Box<Self>, e2: Box<Self>) -> Box<Self> {
        Box::new(Self::Mul(e1, e2))
    }

    fn inv(e: Box<Self>) -> Box<Self> {
        Box::new(Self::Inv(e))
    }

    fn constant(c: Const) -> Box<Self> {
        Box::new(Self::Const(c))
    }

    fn func(name: String, arg: Box<Self>) -> Box<Self> {
        Box::new(Self::Func { name, arg })
    }

    fn var(v: Var) -> Box<Self> {
        Box::new(Self::Var(v))
    }

    fn to_string(&self) -> String {
        match self {
            Self::Add(e1, e2) => format!("({} + {})", e1.to_string(), e2.to_string()),
            Self::Neg(e) => format!("-({})", e.to_string()),
            Self::Mul(e1, e2) => format!("({} * {})", e1.to_string(), e2.to_string()),
            Self::Inv(e) => format!("1/({})", e.to_string()),
            Self::Const(c) => c.to_string(),
            Self::Func { name, arg } => format!("{}({})", name, arg.to_string()),
            Self::Var(v) => v.to_string(),
        }
    }

    fn arg_count(&self) -> u32 {
        match self {
            Self::Const(_) | Self::Var(_) => 0,
            Self::Neg(_) | Self::Inv(_) | Self::Func { .. } => 1,
            Self::Add(_, _) | Self::Mul(_, _) => 2,
        }
    }

    fn diff(self, by: Var) -> Box<Self> {
        match self {
            Self::Const(_) => Self::constant(Const::Numeric(0)),
            Self::Var(v) => {
                if v == by {
                    Self::constant(Const::Numeric(1))
                } else {
                    Self::constant(Const::Numeric(0))
                }
            },
            Self::Add(e1, e2) => Self::add(e1.diff(by), e2.diff(by)),
            Self::Neg(e) => Self::neg(e.diff(by)),
            Self::Mul(e1, e2) => Self::add(
                Self::mul(e1.clone().diff(by), e2.clone()),
                Self::mul(e1, e2.diff(by)),
            ),
            Self::Inv(e) => Self::mul(
                Self::neg(Self::inv(Self::mul(e.clone(), e.clone()))),
                e.diff(by),
            ),
            Self::Func { name, arg } => Self::mul(
                Self::func(format!("{}_{}", name, by.to_string()), arg.clone()),
                arg.diff(by),
            ),
        }
    }

    fn unpack_inv_inv(self) -> Option<Box<Self>> {
        let Self::Inv(inner) = self else { return None };
        let Self::Inv(e) = *inner else { return None };
        Some(e)
    }

    fn uninv(self: Box<Self>) -> Box<Self> {
        let mut expr = self;
        while let Some(inner) = expr.clone().unpack_inv_inv() {
            expr = inner;
        }
        expr
    }

    fn unpack_neg_neg(self) -> Option<Box<Self>> {
        if let Self::Neg(inner) = self && let Self::Neg(e) = *inner {
            Some(e)
        } else {
            None
        }
    }

    fn unneg(self: Box<Self>) -> Box<Self> {
        let mut expr = self;
        while let Some(inner) = expr.clone().unpack_neg_neg() {
            expr = inner;
        }
        expr
    }

    fn substitute(self, name: &str, value: Box<Self>) -> Box<Self> {
        match self {
            Self::Const(Const::Named(ref s)) if s == name => value,
            Self::Const(c) => Self::constant(c),
            Self::Var(v) => Self::var(v),
            Self::Add(e1, e2) => {
                Self::add(e1.substitute(name, value.clone()), e2.substitute(name, value))
            }
            Self::Mul(e1, e2) => {
                Self::mul(e1.substitute(name, value.clone()), e2.substitute(name, value))
            }
            Self::Neg(e) => Self::neg(e.substitute(name, value)),
            Self::Inv(e) => Self::inv(e.substitute(name, value)),
            Self::Func {
                name: fname,
                arg,
            } => Self::func(fname, arg.substitute(name, value)),
        }
    }
}

// Podpunkt 14: użycie systemu w main w celu eliminacji ostrzeżeń
fn main() {
    let x = Var::X;
    let y = Var::Y;
    let z = Var::Z;
    println!("Variables: {}, {}, {}", x.to_string(), y.to_string(), z.to_string());

    let c1 = Const::Numeric(42);
    let c2 = Const::Named("a".to_string());
    println!("Constants: {}, {}", c1.to_string(), c2.to_string());

    let expr = E::add(
        E::mul(E::var(x), E::var(y)),
        E::func("f".into(), E::constant(c2)),
    );
    println!("Expression: {}", expr.to_string());
    println!("Number of arguments in the root: {}", expr.arg_count());

    let d = expr.clone().diff(Var::X);
    println!("Derivative with respect to X: {}", d.to_string());

    let double_inv = E::inv(E::inv(E::var(z)));
    let _ = double_inv.clone().unpack_inv_inv();
    let uninv_expr = double_inv.uninv();
    println!("Simplified inverse: {}", uninv_expr.to_string());

    let double_neg = E::neg(E::neg(E::var(y)));
    let _ = double_neg.clone().unpack_neg_neg();
    let unneg_expr = double_neg.unneg();
    println!("Simplified negation: {}", unneg_expr.to_string());

    let subst = expr.substitute("a", E::constant(c1));
    println!("After substitution: {}", subst.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_to_string() {
        let c_num = Const::Numeric(42);
        let c_name = Const::Named("a".into());
        assert_eq!(c_num.to_string(), "42");
        assert_eq!(c_name.to_string(), "a");
    }

    #[test]
    fn test_var_to_string() {
        assert_eq!(Var::X.to_string(), "X");
        assert_eq!(Var::Y.to_string(), "Y");
        assert_eq!(Var::Z.to_string(), "Z");
    }

    #[test]
    fn test_builder_constant_var() {
        let e_const = E::constant(Const::Numeric(5));
        let e_var = E::var(Var::X);
        assert_eq!(e_const.to_string(), "5");
        assert_eq!(e_var.to_string(), "X");
    }

    #[test]
    fn test_builder_add() {
        let expr = E::add(E::constant(Const::Numeric(2)), E::var(Var::X));
        assert_eq!(expr.to_string(), "(2 + X)");
    }

    #[test]
    fn test_builder_neg() {
        let expr = E::neg(E::var(Var::X));
        assert_eq!(expr.to_string(), "-(X)");
    }

    #[test]
    fn test_builder_mul() {
        let expr = E::mul(E::var(Var::X), E::var(Var::Y));
        assert_eq!(expr.to_string(), "(X * Y)");
    }

    #[test]
    fn test_builder_inv() {
        let expr = E::inv(E::var(Var::X));
        assert_eq!(expr.to_string(), "1/(X)");
    }

    #[test]
    fn test_builder_func() {
        let expr = E::func("f".into(), E::var(Var::X));
        assert_eq!(expr.to_string(), "f(X)");
    }

    #[test]
    fn test_expr_to_string_complex() {
        let expr1 = E::add(E::constant(Const::Numeric(2)), E::var(Var::X));
        let expr2 = E::mul(E::neg(E::var(Var::Y)), E::inv(E::var(Var::Z)));
        let complex = E::add(
            E::func("f".into(), expr1.clone()),
            E::func("g".into(), expr2.clone()),
        );
        assert_eq!(complex.to_string(), "(f((2 + X)) + g((-(Y) * 1/(Z))))");
    }

    #[test]
    fn test_diff_add_vars() {
        let expr = E::add(E::var(Var::X), E::var(Var::Y));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "(1 + 0)");
    }

    #[test]
    fn test_unpack_inv_inv() {
        let double_inv = E::inv(E::inv(E::var(Var::X)));
        let inner = double_inv.clone().unpack_inv_inv().unwrap();
        assert_eq!(inner.to_string(), "X");
    }

    #[test]
    fn test_unpack_neg_neg() {
        let double_neg = E::neg(E::neg(E::neg(E::neg(E::neg(E::var(Var::Y))))));
        let inner = double_neg.clone().unneg();
        assert_eq!(inner.to_string(), "-(Y)");
    }

    #[test]
    fn test_simplify_double_inv() {
        let double_inv = E::inv(E::inv(E::var(Var::X)));
        let simplified = double_inv.uninv();
        assert_eq!(simplified.to_string(), "X");
    }

    #[test]
    fn test_simplify_double_neg() {
        let double_neg = E::neg(E::neg(E::var(Var::X)));
        let simplified = double_neg.unneg();
        assert_eq!(simplified.to_string(), "X");
    }

    #[test]
    fn test_substitute_named_constant() {
        let expr = E::add(E::constant(Const::Named("a".into())), E::var(Var::X));
        let substituted = expr.substitute("a", E::constant(Const::Numeric(10)));
        assert_eq!(substituted.to_string(), "(10 + X)");
    }

    #[test]
    fn test_substitute_deep() {
        let expr = E::mul(
            E::constant(Const::Named("a".into())),
            E::func("f".into(), E::constant(Const::Named("a".into()))),
        );
        let substituted = expr.substitute("a", E::constant(Const::Numeric(3)));
        assert_eq!(substituted.to_string(), "(3 * f(3))");
    }

    #[test]
    fn test_diff_neg() {
        let expr = E::neg(E::var(Var::X));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "-(1)");
    }

    #[test]
    fn test_diff_mul() {
        let expr = E::mul(E::var(Var::X), E::var(Var::Y));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "((1 * Y) + (X * 0))");
    }

    #[test]
    fn test_diff_inv() {
        let expr = E::inv(E::var(Var::X));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "(-(1/((X * X))) * 1)");
    }

    #[test]
    fn test_diff_const_numeric() {
        let expr = E::constant(Const::Numeric(7));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "0");
    }

    #[test]
    fn test_diff_const_named() {
        let expr = E::constant(Const::Named("a".into()));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "0");
    }

    #[test]
    fn test_diff_func() {
        let expr = E::func("f".into(), E::var(Var::X));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "(f_X(X) * 1)");
    }

    #[test]
    fn test_diff_var_same() {
        let d = E::var(Var::X).diff(Var::X);
        assert_eq!(d.to_string(), "1");
    }

    #[test]
    fn test_diff_var_other() {
        let d = E::var(Var::Y).diff(Var::X);
        assert_eq!(d.to_string(), "0");
    }

    #[test]
    fn test_diff_big_expression() {
        // (((X + -(Y)) * 1/(Z)) + (f((X * Y)) + g(1/(X))))
        let part1 = E::add(E::var(Var::X), E::neg(E::var(Var::Y)));
        let part2 = E::inv(E::var(Var::Z));
        let a = E::mul(part1.clone(), part2.clone());
        let xy = E::mul(E::var(Var::X), E::var(Var::Y));
        let b = E::func("f".into(), xy);
        let inv_x = E::inv(E::var(Var::X));
        let c = E::func("g".into(), inv_x);
        let big = E::add(a.clone(), E::add(b.clone(), c.clone()));

        assert_eq!(
            big.to_string(),
            "(((X + -(Y)) * 1/(Z)) + (f((X * Y)) + g(1/(X))))"
        );

        let d = big.diff(Var::X);
        assert_eq!(
            d.to_string(),
            "((((1 + -(0)) * 1/(Z)) + ((X + -(Y)) * (-(1/((Z * Z))) * 0))) + ((f_X((X * Y)) * ((1 * Y) + (X * 0))) + (g_X(1/(X)) * (-(1/((X * X))) * 1))))"
        );
    }

    #[test]
    fn test_arg_count_zeroary() {
        assert_eq!(E::constant(Const::Numeric(1)).arg_count(), 0);
        assert_eq!(E::var(Var::X).arg_count(), 0);
    }

    #[test]
    fn test_arg_count_unary() {
        assert_eq!(E::neg(E::var(Var::X)).arg_count(), 1);
        assert_eq!(E::inv(E::var(Var::X)).arg_count(), 1);
        assert_eq!(E::func("f".into(), E::var(Var::X)).arg_count(), 1);
    }

    #[test]
    fn test_arg_count_binary() {
        assert_eq!(E::add(E::var(Var::X), E::var(Var::Y)).arg_count(), 2);
        assert_eq!(E::mul(E::var(Var::X), E::var(Var::Z)).arg_count(), 2);
    }
}
