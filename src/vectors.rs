
use std::{fmt::Display, ops::{Mul, AddAssign}};

#[allow(unused_imports)]
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
    pub fn new(content: Vec<T>, column_or_row: bool) -> Self {
        return Vector { 
            content: content, 
            column_or_row 
        };
        
    }
    /// Push new values into the vector
    pub fn push(&mut self, value: T) {
        self.content.push(value);
    }
    /// Switch between column and row vector
    pub fn transpose(&mut self) {
        self.column_or_row = !self.column_or_row;
    }
}

impl<T: Display> Display for Vector<T> {
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

impl<T: AddAssign + Default + Mul + Mul<Output = T> + Copy + Display> Mul for Vector<T>{
    type Output = T;

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