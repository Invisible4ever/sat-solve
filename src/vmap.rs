use crate::types::{Var, AVar, AValue, Literal, Clause, };
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Vmap {
    vals: Vec<AVar>
}


impl Index<Var> for Vmap {
    type Output = AVar;

    fn index(&self, index: Var) -> &Self::Output {
        & (self.vals[index - 1])
    }
}

impl IndexMut<Var> for Vmap {
    fn index_mut(&mut self, index: Var) -> &mut Self::Output {
        &mut (self.vals[index - 1])
    }
}

impl Vmap {
    pub fn new(size: usize) -> Self {
        let mut vals = Vec::with_capacity(size);
        for ind in 0..size {
            vals.push(AVar::from_var(ind + 1));
        }

        Self { vals }
    }
    
    pub fn value(&self, lit: &Literal) -> AValue {
        let val = self[lit.var].val;

        if lit.neg { !val } else { val }
    }

    pub fn split_var(&mut self, dec_lvl: u32) -> Option<Literal> {
        let avar = self.vals.iter_mut().find(|avar| avar.val == AValue::AUndef)?;
        // TODO: No reason to use True over False.
        avar.val = AValue::ATrue;
        avar.dec_lvl = dec_lvl;
        Some(Literal { var: avar.var, neg: false })
    }
    
}


#[test]
fn test_indexing() {
    let vmap = Vmap::new(2);
    vmap[1];
    vmap[2];
}
