use crate::evaluator::{IPL_Library, Instance};
use std::ops::Index;
use std::ops::{Add, Div, Mul, Sub};
use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    List(Vec<Value>),
    Bool(bool),
    Str(String),
    Path(PathBuf),
    Instance(Instance),
    #[allow(non_camel_case_types)] // For readability
    IPL_Library(IPL_Library),
    None,
}

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            Value::List(vec) => &vec[idx],
            _ => panic!("Indexing only supported on Value::List"),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),
            _ => Value::None,
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => Value::None,
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => Value::None,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Number(_), Value::Number(0.0)) => Value::None,
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => Value::None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::None, Value::None) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::Str(a), Value::Str(b)) => a.partial_cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

// Define methods for Value type conversions and utilities
impl Value {
    pub fn as_f64(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Value::Str(s) => s.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    pub fn as_usize(&self) -> usize {
        match self {
            Value::Number(n) => *n as usize,
            Value::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::None => false,
            _ => true,
        }
    }
    pub fn to_string_value(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::Path(p) => p.to_str().unwrap_or("").to_string(),
            Value::None => "None".into(),
            _ => "".to_string(),
        }
    }
    pub fn length(&self) -> usize {
        match self {
            Value::List(v) => v.len(),
            Value::Str(s) => s.len(),
            _ => 0,
        }
    }
    pub fn iter(&self) -> Box<dyn Iterator<Item = &Value> + '_> {
        match self {
            Value::List(v) => Box::new(v.iter()),
            _ => Box::new(std::iter::empty()),
        }
    }
    pub fn get_instance(&self) -> Option<Instance> {
        match self {
            Value::Instance(inst) => Some(inst.clone()),
            _ => None,
        }
    }
    pub fn is_none_value(&self) -> bool {
        matches!(self, Value::None)
    }
}