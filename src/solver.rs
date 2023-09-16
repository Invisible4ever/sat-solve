use std::collections::VecDeque;

use crate::types::*;
use crate::vmap::Vmap;

#[derive(Debug)]
pub struct Solver {
    dec_lvl: u32,
    assigns: Vmap, 
    clauses: Vec<Clause>,
    learned: Vec<Clause>,
    trail: Vec<Literal>,
    trail_dec_levels: Vec<usize>,
}

fn cl_sat(clause: &Clause, assigns: &Vmap) -> bool {
    clause.lits.iter().any(|lit| assigns.value(lit) == AValue::ATrue)
}

fn cl_unsat(clause: &Clause, assigns: &Vmap) -> bool {
    clause.lits.iter().all(|lit| assigns.value(lit) == AValue::AFalse)
}

#[test]
fn test_cl_unsat() {
    let cl = Clause::new(1, vec![1,-2,-3]);
    let mut vmap = Vmap::new(3);
    vmap[1].val = AValue::AFalse;
    vmap[2].val = AValue::ATrue;
    assert!(!cl_unsat(&cl, &vmap));

    vmap[3].val = AValue::ATrue;
    assert!(cl_unsat(&cl, &vmap));
}

fn is_unit(clause: &Clause, assigns: &Vmap) -> Option<Literal> {
    let mut unit_lit = None;
    for lit in clause.iter() {
        match assigns.value(lit) {
            AValue::AFalse => continue,
            AValue::ATrue => return None,
            AValue::AUndef => {
                if unit_lit.is_some() { return None; }
                unit_lit = Some(*lit)
            },
        }
    }
    unit_lit
}

#[test]
fn test_is_unit() {
    let cl = Clause::new(1, vec![1,-2,-3]);
    let mut vmap = Vmap::new(3);
    vmap[1].val = AValue::AFalse;
    vmap[2].val = AValue::ATrue;

    assert_eq!(is_unit(&cl, &vmap), Some((-3).into()));

    let simple_cl = Clause::new(2, vec![3]);
    assert_eq!(is_unit(&simple_cl, &vmap), Some((3).into()));
}

impl Solver {
    pub fn new(clauses: Vec<Clause>, vars: usize) -> Self {
        Self { 
            dec_lvl: 0,
            assigns: Vmap::new(vars),
            clauses,
            learned: vec![],
            trail: vec![],
            trail_dec_levels: vec![0],
        }
    }

    pub fn solve(&mut self) -> Result<(), ()> {
        let mut new_clause: Clause;

        while self.clauses.iter().any(|cl| !cl_sat(cl, &self.assigns)) {
            let prop_res = self.unit_propagation();

            match prop_res {
                Ok(_) => {
                    self.dec_lvl += 1;
                    let split_opt = self.assigns.split_var(self.dec_lvl);
                    if split_opt.is_none() {
                        break; // while condition is now false
                    }
                    
                    let split_var = split_opt.unwrap();
                    self.trail.push(split_var);

                    self.trail_dec_levels.push(self.trail.len());
                },
                Err(conflict) => {
                    if self.dec_lvl == 0 {
                        return Err(());
                    }

                    let mut out_clause = self.add_empty_clause();
                    let pop_to = self.extract_clause(self.get_clause(conflict), &mut out_clause);

                    self.learned.push(out_clause);
                    self.backtrack(pop_to);
                }
            };
        }
        Ok(())
    }

    fn unit_propagation(&mut self) -> Result<(),usize> {
        // TODO: Watchsets!
        loop { 
            let mut changes = false;
            for (cl_i, clause) in self.clauses.iter().chain(self.learned.iter()).enumerate() {
                if let Some(lit) = is_unit(clause, &self.assigns) {
                    self.trail.push(lit);
                    self.assigns[lit.var] = self.assign(lit, cl_i);
                    changes = true;
                    continue;
                }

                if cl_unsat(clause, &self.assigns) {
                    return Err(cl_i); 
                }
            }
            if !changes {break;}
        }

        Ok(())
    }

    fn extract_clause(&self, start_conflict: &Clause, out_clause: &mut Clause) -> u32 {
        // invar: self.trail.len() >= 1
        let mut found = vec![];
        let mut reason = start_conflict.reason(); 
        let mut reason_lit : Literal;
        let mut last = self.trail.len() - 1;
        let mut backtrack_lvl = 0;
        
        //counts the number of literals in found that are of current dec_lvl
        let mut counter = 0;

            
        // Find UIP
        loop {
            for lit in &reason {
                if found.contains(&lit.var) { continue; }
                found.push(lit.var);

                let avar = self.assigns[lit.var];
                if avar.dec_lvl == self.dec_lvl {
                    counter += 1;
                    continue;
                } else if avar.dec_lvl > 0 {
                    out_clause.lits.push(*lit);
                    if backtrack_lvl < avar.dec_lvl {
                        backtrack_lvl = avar.dec_lvl;
                    }
                }
            }

            loop {
                // TODO: Handle None.
                reason_lit = self.trail[last];
                last -= if last == 0 {0} else {1};

                if found.contains(&reason_lit.var) {
                    break;
                }
            }

            counter -= 1;
            if counter == 0 {
                break;
            }

            let ante = self.assigns[reason_lit.var].ante.unwrap();
            reason = self.get_clause(ante).reason_for(reason_lit.var);
        }

        out_clause.lits.push(!reason_lit); 
        backtrack_lvl
    }

    fn assign<T>(&self, lit: Literal, ante: T) -> AVar where 
            Option<usize>:From<T>,
        {
        let ante = Option::from(ante);
        AVar {
            var: lit.var,
            val: (!lit.neg).into(),
            dec_lvl: self.dec_lvl,
            ante
        }
    }

    pub fn add_empty_clause(&self) -> Clause {
        Clause {
            id: self.clauses.len() + self.learned.len(),
            lits: vec![]
        }
    }

    pub fn add_clause(&mut self, literals: Vec<i32>) {
        self.clauses.push(Clause::new(self.clauses.len(), literals));
    }

    #[inline]
    pub fn get_clause(&self, id: usize) -> &Clause {
        if id < self.clauses.len() {
            &self.clauses[id]
        } else {
            &self.learned[id - self.clauses.len()]
        }
    }

    fn pop_one(&mut self) {
        let last = self.trail.last().unwrap();

        self.assigns[last.var].clear();
        self.trail.pop();
    }

    pub fn backtrack(&mut self, level: u32) {
        let new_last = self.trail_dec_levels[level as usize];
        self.trail_dec_levels.truncate(level as usize);
        
        for lit in self.trail.iter().skip(new_last) {
            self.assigns[lit.var].clear();
        }

        self.trail.truncate(new_last);
        self.dec_lvl = level;
    } 
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
