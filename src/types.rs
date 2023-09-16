use std::ops;

use crate::vmap::Vmap;

pub type Var = usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Literal {
    pub var: Var,
    pub neg: bool
}

impl ops::Not for Literal {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.neg = !self.neg;
        self
    }
}

impl From<i32> for Literal {
    fn from(lit: i32) -> Self {
        assert_ne!(lit, 0);
        Literal { 
            var: lit.abs() as usize, neg: lit < 0
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AValue {
    ATrue,
    AFalse,
    AUndef
}

impl From<bool> for AValue {
    fn from(val: bool) -> Self {
        if val {
            AValue::ATrue
        } else {
            AValue::AFalse
        }
    }
}

impl ops::Not for AValue {
    type Output = AValue;

    fn not(self) -> Self::Output {
        match self {
            Self::ATrue => Self::AFalse,
            Self::AFalse => Self::ATrue,
            Self::AUndef => Self::AUndef 
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AVar {
    pub var: Var,
    pub val: AValue,
    pub ante: Option<usize>,
    pub dec_lvl: u32 
}

impl AVar {
    pub fn from_var(var: Var) -> Self {
        Self { 
            var,
            val: AValue::AUndef,
            ante: None,
            dec_lvl: 0
        }
    }

    pub fn clear(&mut self) {
        self.val = AValue::AUndef;
        self.ante = None;
        self.dec_lvl = 0;
    }
}

#[derive(Debug)]
pub struct Clause {
    pub id: usize,
    pub lits: Vec<Literal>,
}


impl Clause {
    pub fn new(id: usize, lits: Vec<i32>) -> Self {
        Clause { id, lits: lits.into_iter().map(From::from).collect() }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Literal> {
        self.lits.iter()
    }

    pub fn reason_for(&self, var: Var) -> Vec<Literal> {
        self.lits.iter().filter(|lit| lit.var != var).map(|lit| !*lit).collect()
    }

    pub fn reason(&self) -> Vec<Literal> {
        self.lits.iter().map(|lit| !*lit).collect()
    }

}

