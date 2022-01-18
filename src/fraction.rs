/*!
# Fractions
This is the fraction module.

# Examples
```rust
use lemonmath::fraction::Fraction;

// Create a fraction from a numerator and denominator
let x = Fraction::new(1, 2);
// Create a fraction from a float
let y = Fraction::from_float(10.2082);

// You can display fractions
println!("{}", x);
aseert_eq!(format!("{}", x), "1/2");

// You can add fractions
let z = x + y;
assert_eq!(z, Fraction::new(11, 2));

// You can subtract fractions
let z = x - y;
assert_eq!(z, Fraction::new(-9, 2));

// You can multiply fractions
let z = x * y;
assert_eq!(z, Fraction::new(10, 1));

// You can divide fractions
let z = x / y;
assert_eq!(z, Fraction::new(5, 1));

// You can add assign fractions
let mut x = Fraction::new(1, 2);
x += y;
assert_eq!(x, Fraction::new(11, 2));

// You can sub assign fractions
let mut x = Fraction::new(1, 2);
x -= y;
assert_eq!(x, Fraction::new(-9, 2));

// You can mul assign fractions
let mut x = Fraction::new(1, 2);
x *= y;
assert_eq!(x, Fraction::new(10, 1));

// You can div assign fractions
let mut x = Fraction::new(1, 2);
x /= y;
assert_eq!(x, Fraction::new(5, 1));
```
*/

use std::{ops::{Add, Sub, Mul, Div, AddAssign, DivAssign, SubAssign, MulAssign}, fmt::{Formatter, Display}};

use crate::helper::{GetDecimal, GCD};

#[test]
fn fraction_test() {
    let x = Fraction::from_float(10.2044982);
    let y = Fraction::from_float(1.0);
    println!("{}", x);
}

/// A struct to replace floats
/// 
/// # Examples
/// ```
/// use lemonmath::fraction::Fraction;
/// 
/// let x = Fraction::from_float(10.2082);
/// 
/// println!("{}", x);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fraction {
    pub numerator: i128,
    pub denominator: i128,
}

impl Fraction {
    /// This creates a new Fraction from a numerator and denominator.
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// let x = Fraction::new(1, 2);
    /// 
    /// assert_eq!(x.numerator, 1);
    /// assert_eq!(x.denominator, 2);
    /// ```
    pub fn new(numerator: i128, denominator: i128) -> Self {
        return Fraction {
            numerator,
            denominator: denominator.abs(),
        }.reduce();
    }
    /// This creates a new Fraction from a float
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::from_float(1.0);
    /// 
    /// assert_eq!(x.numerator, 1);
    /// assert_eq!(x.denominator, 1);
    /// ```
    pub fn from_float(value: f64) -> Self {
        let ten_pow = 10_u32.pow(format!("{}", value.get_decimal()).len() as u32);
        let numerator = (value.trunc() as i128 * ten_pow as i128) + (value.get_decimal() as i128);
        let denominator = ten_pow as i128;
        return Fraction::new(numerator, denominator).reduce();
    }
    /// This adds two fractions together
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// assert_eq!(x.add_number(y), Fraction::new(7, 6));
    /// ```
    pub fn add_number(&self, other: Self) -> Self {
        let mut numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        let mut denominator = self.denominator * other.denominator;
        let gcd = numerator.gcd(denominator);
        numerator /= gcd;
        denominator /= gcd;
        return Fraction::new(numerator, denominator);
    }
    /// This multiplies two fractions together
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// assert_eq!(x.mul_number(y), Fraction::new(1, 3));
    /// ```
    pub fn mul_number(&self, other: Self) -> Self {
        let mut numerator = self.numerator * other.numerator;
        let mut denominator = self.denominator * other.denominator;
        let gcd = numerator.gcd(denominator);
        numerator /= gcd;
        denominator /= gcd;
        return Fraction::new(numerator, denominator);
    }
    /// This divides two fractions
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// assert_eq!(x.div_number(y), Fraction::new(3, 4));
    /// ```
    pub fn div_number(&self, other: Self) -> Self {
        return self.mul(Fraction::new(other.denominator, other.numerator));
    }
    /// This subtracts two fractions
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// assert_eq!(x.sub_number(y), Fraction::new(-1, 6));
    /// ```
    pub fn sub_number(&self, other: Self) -> Self {
        return self.add(Fraction::new(-other.numerator, other.denominator));
    }
    /// This reduces the fraction to its lowest terms
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::new(2, 4);
    /// 
    /// assert_eq!(x.reduce(), Fraction::new(1, 2));
    /// ```
    pub fn reduce(&self) -> Self {
        let mut numerator = self.numerator;
        let mut denominator = self.denominator;
        let gcd = numerator.gcd(denominator);
        numerator /= gcd;
        denominator /= gcd;
        return Fraction {
            numerator, 
            denominator
        };
    }
}

impl Add for Fraction {
    type Output = Self;
    
    /// This adds two fractions together
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// assert_eq!(x + y, Fraction::new(7, 6));
    /// ```
    fn add(self, other: Self) -> Self {
        return self.add_number(other);
    }
}

impl Sub for Fraction {
    type Output = Self;
    
    /// This subtracts two fractions
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// assert_eq!(x - y, Fraction::new(-1, 6));
    /// ```
    fn sub(self, other: Self) -> Self {
        return self.sub_number(other);
    }
}

impl Mul for Fraction {
    type Output = Self;

    /// This multiplies two fractions together
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// assert_eq!(x * y, Fraction::new(1, 3));
    /// ```
    fn mul(self, other: Self) -> Self {
        return self.mul_number(other);
    }
}

impl Div for Fraction {
    type Output = Self;
    
    /// This divides two fractions
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// assert_eq!(x / y, Fraction::new(3, 4));
    /// ```
    fn div(self, other: Self) -> Self {
        return self.div_number(other);
    }
}

impl Display for Fraction {
    /// This displays the fraction as a string
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::new(1, 2);
    /// 
    /// assert_eq!(format!("{}", x), "1/2");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let denominator = match self.denominator {
            1 => String::new(),
            _ => String::from("/".to_owned() + &self.denominator.to_string()),
        };
        write!(f, "{}{}", self.numerator, denominator)
    }
}

impl AddAssign for Fraction {
    /// This adds two fractions together and then assigns the result to the original fraction
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let mut x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// x += y;
    /// assert_eq!(x, Fraction::new(7, 6));
    /// ```
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Fraction {
    /// This subtracts two fractions and then assigns the result to the original fraction
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let mut x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// x -= y;
    /// assert_eq!(x, Fraction::new(-1, 6));
    /// ```
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Fraction {
    /// This multiplies two fractions and then assigns the result to the original fraction
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    ///     
    /// let mut x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// x *= y;
    /// assert_eq!(x, Fraction::new(1, 3));
    /// ```
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl DivAssign for Fraction {
    /// This divides two fractions and then assigns the result to the original fraction
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let mut x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    /// 
    /// x /= y;
    /// assert_eq!(x, Fraction::new(3, 4));
    /// ```
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl Default for Fraction {
    /// This returns a fraction with a numerator of 0 and a denominator of 1
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    /// 
    /// let x = Fraction::default();
    /// 
    /// assert_eq!(x, Fraction::new(0, 1));
    /// ```
    fn default() -> Self {
        return Fraction::new(0, 1);
    }
}