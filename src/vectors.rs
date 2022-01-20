/*!
# Vectors
Math Vectors;

# Examples
```rust
use lemonmath::vectors::Vector;

// Create Vector from a list of numbers
let x = Vector::new(vec![1, 2, 3], true);

assert_eq!(x.content, vec![1.0, 2.0, 3.0, 4.0, 5.0]);

// Push a new element to the vector
let mut vector = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);

vector.push(6.0);

assert_eq!(vector.content, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

// Transpose example
let mut vector1 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
let vector2 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], false);

vector1.transpose();

assert_eq!(format!("{}", vector1), format!("{}", vector2));

// Display Trait
let x = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);

assert_eq!(format!("{}", x), "[ 1.0 2.0 3.0 4.0 5.0 ]");

// Dot Product
let vector1 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
let vector2 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], false);

assert_eq!(vector1 * vector2, 55.0);
```
*/

use std::{fmt::Display, ops::{Mul, AddAssign, Add, Sub, Div}};

use crate::helper::VecToFraction;

#[test]
pub fn vector_test() {
    let vector1 = Vector::new(vec![1.32, 2.0, 3.432, 4.0, 5.0].to_fraction(), true);
    let mut vector2 = Vector::new(vec![1.0, 2.0, 3.0, 4.53, 5.0].to_fraction(), true);
    vector2.transpose();
    println!("{}\n{}", vector1, vector2);
    println!("{}", vector2 * vector1);
}

/// Math Vector
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Vector<T> {
    pub content: Vec<T>,
    column_or_row: bool
}

impl<T> Vector<T> {
    /// Create the Vector  
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::vectors::Vector;
    /// 
    /// // Vector::new(Vector<T>, bool);
    /// // Create a Vector from a list of numbers
    /// // The bool is to determine if the vector is a column or row
    /// let x = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
    /// 
    /// assert_eq!(x.content, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    /// ```
    pub fn new(content: Vec<T>, column_or_row: bool) -> Self {
        return Vector { 
            content: content, 
            column_or_row 
        };
        
    }
    /// Push new values into the vector
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::vectors::Vector;
    /// 
    /// let mut vector = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
    /// 
    /// vector.push(6.0);
    /// 
    /// assert_eq!(vector.content, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    /// ```
    pub fn push(&mut self, value: T) {
        self.content.push(value);
    }
    /// Switch between column and row vector
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::vectors::Vector;
    /// 
    /// let mut vector1 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
    /// let vector2 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], false);
    /// 
    /// vector1.transpose();
    /// 
    /// assert_eq!(format!("{}", vector1), format!("{}", vector2));
    /// ```
    pub fn transpose(&mut self) {
        self.column_or_row = !self.column_or_row;
    }
}

impl<T: Display> Display for Vector<T> {
    /// Display the vector
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::vectors::Vector;
    /// 
    /// let x = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
    /// 
    /// assert_eq!(format!("{}", x), "[ 1.0 2.0 3.0 4.0 5.0 ]");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut final_str = String::new();
        if self.column_or_row == true {
            let mut max_length = 1;
            for x in &self.content {
                if format!("{}", x).len() > max_length {
                    max_length = format!("{}", x).len();
                }
            }
            for x in (&self.content).into_iter().enumerate() {
                let mut padding = String::new();
                for _ in 0..(max_length - format!("{}", x.1).len()) {
                    padding.push(' ');
                }
                if x.0 == 0 {
                    final_str.push_str(&format!("⎡ {}{} ⎤\n", x.1, padding));
                } else if x.0 + 1 == self.content.len() {
                    final_str.push_str(&format!("⎣ {}{} ⎦", x.1, padding));
                } else {
                    final_str.push_str(&format!("⎢ {}{} ⎥\n", x.1, padding));
                }
            }
        } else {
            final_str.push('[');
            final_str.push(' ');
            for x in &self.content {
                final_str.push_str(format!("{} ", x).as_str());
            }
            final_str.push(']');
        }
        return f.write_str(final_str.as_str());
    }
}

impl<T: Add + Add<Output = T> + Copy> Add for Vector<T> {
    type Output = Self;
    
    /// Add two vectors
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::vectors::Vector;
    /// 
    /// let vector1 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
    /// let vector2 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
    /// 
    /// assert_eq!(vector1 + vector2, Vector::new(vec![2.0, 4.0, 6.0, 8.0, 10.0], true));
    /// ```
    fn add(self, other: Self) -> Self {
        let mut result = Vector::new(vec![], self.column_or_row);
        for x in 0..self.content.len() {
            result.content.push(self.content[x] + other.content[x]);
        }
        return result;
    }
}

impl<T: Sub + Sub<Output = T> + Copy> Sub for Vector<T> {
    type Output = Self;
    
    /// Subtract two vectors
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::vectors::Vector;
    /// 
    /// let vector1 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
    /// let vector2 = Vector::new(vec![1.0, 21.0, 56.0, 4.00, 52.0], true);
    /// 
    /// assert_eq!(vector1 - vector2, Vector::new(vec![0.0, -19.0, -53.0, 0.0, -47.0], true));
    /// ```
    fn sub(self, other: Self) -> Self {
        let mut result = Vector::new(vec![], self.column_or_row);
        for x in 0..self.content.len() {
            result.content.push(self.content[x] - other.content[x]);
        }
        return result;
    }
}

impl<T: AddAssign + Default + Mul + Mul<Output = T> + Copy + Display> Mul for Vector<T>{
    type Output = T;

    /// Dot Product
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::vectors::Vector;
    /// 
    /// let vector1 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
    /// let vector2 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], false);
    /// 
    /// assert_eq!(vector1 * vector2, 55.0);
    /// ```
    fn mul(self, other: Self) -> Self::Output {
        let mut result = T::default();
        if self.content.len() != other.content.len() {
            panic!("Vectors must have the same length");
        }
        if self.column_or_row != other.column_or_row {
            for x in 0..self.content.len() {
                result += self.content[x] * other.content[x];
            }
        } else {
            panic!("Can't multiply two vectors with the same orientation");
        }
        return result;
    }
}

impl<T: From<u8> + Mul + Div + Div<Output = T> + Mul<Output = T> + Copy> Div for Vector<T> {
    type Output = Self;

    /// Divide a vector by a vector
    /// 
    /// # Examples
    /// ```
    /// use lemonmath::vectors::Vector;
    /// 
    /// let vector1 = Vector::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], true);
    /// let vector2 = Vector::new(vec![2.0, 3.0, 4.0, 5.0, 6.0], false);
    ///
    /// assert_eq!(vector1 / vector2, Vector::new(vec![0.5, 0.6666666666666666, 0.75, 0.8, 0.8333333333333334], true)); 
    /// ```
    fn div(self, other: Self::Output) -> Self::Output {
        let mut result = Vector::new(vec![], self.column_or_row);
        for x in 0..self.content.len() {
            result.content.push(self.content[x] * 1u8.into()/other.content[x]);
        }
        return result;
    }
}