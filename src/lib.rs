use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Ty {
    Var(u32),
    Bool,
    Nat,
    Arr(Rc<Ty>, Rc<Ty>),
}

impl Ty {
    pub fn arr(ty1: Rc<Ty>, ty2: Rc<Ty>) -> Rc<Ty> {
        Rc::new(Ty::Arr(ty1, ty2))
    }

    pub fn var(var: u32) -> Rc<Ty> {
        Rc::new(Ty::Var(var))
    }

    pub fn bool() -> Rc<Ty> {
        Rc::new(Ty::Bool)
    }
    
    pub fn nat() -> Rc<Ty> {
        Rc::new(Ty::Nat)
    }

    pub fn compose(s1: HashMap<u32, Rc<Ty>>, s2: HashMap<u32, Rc<Ty>>) -> HashMap<u32, Rc<Ty>> {
        let mut composed = HashMap::new();
        let s1_keys = s1.keys().collect::<HashSet<&u32>>();
    
        for (var, ty) in &s1 {
            composed.insert(*var, Ty::apply(ty, &s2));
        }
    
        for (var, ty) in &s2 {
            if !s1_keys.contains(var) {
                composed.insert(*var, ty.clone());
            }
        }
    
        composed
    }

    pub fn apply(ty: &Rc<Ty>, s: &HashMap<u32, Rc<Ty>>) -> Rc<Ty> {
        let s_keys = s.keys().collect::<HashSet<&u32>>();
    
        match apply_rec(ty, s, &s_keys) {
            Some(ty_) => ty_.clone(),
            None => ty.clone(),
        }
    }
}

fn apply_rec(ty: &Rc<Ty>, s: &HashMap<u32, Rc<Ty>>, s_keys: &HashSet<&u32>) -> Option<Rc<Ty>> {
    match **ty {
        Ty::Var(ref var) => {
            if s_keys.contains(&var) {
                Some(s[var].clone())
            } else {
                None
            }
        },
        Ty::Bool => None,
        Ty::Nat => None,
        Ty::Arr(ref ty1, ref ty2) => {
            let app1 = apply_rec(ty1, s, s_keys);
            let app2 = apply_rec(ty2, s, s_keys);

            match (app1, app2) {
                (None, None) => Some(ty.clone()),
                (Some(ref ty_), None) => Some(Rc::new(Ty::Arr(ty_.clone(), ty2.clone()))),
                (None, Some(ref ty_)) => Some(Rc::new(Ty::Arr(ty1.clone(), ty_.clone()))),
                (Some(ref ty1_), Some(ref ty2_)) => Some(Rc::new(Ty::Arr(ty1_.clone(), ty2_.clone()))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
