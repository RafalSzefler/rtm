use rust_decimal::Decimal;

const DECIMAL_PRECISION: u32 = 4;

/// Represents a decimal amount with a fixed precision 4.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Amount {
    value: Decimal,
}

impl Amount {
    pub fn zero() -> Self {
        Self::new(Decimal::from(0))
    }

    fn new(mut value: Decimal) -> Self {
        value.rescale(DECIMAL_PRECISION);
        Self { value }
    }
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i32> for Amount {
    fn from(value: i32) -> Self {
        Self::new(value.into())
    }
}

impl From<u32> for Amount {
    fn from(value: u32) -> Self {
        Self::new(value.into())
    }
}

impl Default for Amount {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct InvalidNumericalStringError;

impl TryFrom<&str> for Amount {
    type Error = InvalidNumericalStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Decimal::try_from(value) {
            Ok(result) => Ok(Self::new(result)),
            Err(_) => Err(InvalidNumericalStringError),
        }
    }
}

macro_rules! impl_binary_op {
    ( $op: ty, $method: ident ) => {
        impl $op for Amount {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self {
                Self::new(self.value.$method(rhs.value))
            }
        }
    };
}

macro_rules! impl_unary_op {
    ( $op: ty, $method: ident ) => {
        impl $op for Amount {
            fn $method(&mut self, rhs: Self) {
                self.value.$method(rhs.value);
            }
        }
    };
}

impl_binary_op!(std::ops::Add, add);
impl_binary_op!(std::ops::Sub, sub);
impl_binary_op!(std::ops::Mul, mul);
impl_binary_op!(std::ops::Div, div);
impl_unary_op!(std::ops::AddAssign, add_assign);
impl_unary_op!(std::ops::SubAssign, sub_assign);
impl_unary_op!(std::ops::MulAssign, mul_assign);
impl_unary_op!(std::ops::DivAssign, div_assign);

impl std::ops::Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(self.value.neg())
    }
}
