use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

pub type Subs = HashMap<u32, Rc<Ty>>;

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

    pub fn compose(s1: &Subs, s2: &Subs) -> Subs {
        let mut composed = HashMap::new();
        let s1_keys = s1.keys().collect::<HashSet<&u32>>();
    
        for (var, ty) in s1 {
            composed.insert(*var, Ty::apply(ty, &s2));
        }
    
        for (var, ty) in s2 {
            if !s1_keys.contains(var) {
                composed.insert(*var, ty.clone());
            }
        }
    
        composed
    }

    pub fn apply(ty: &Rc<Ty>, s: &Subs) -> Rc<Ty> {
        let s_keys = s.keys().collect::<HashSet<&u32>>();
    
        match apply_rec(ty, s, &s_keys) {
            Some(ty_) => ty_.clone(),
            None => ty.clone(),
        }
    }
}

fn apply_rec(ty: &Rc<Ty>, s: &Subs, s_keys: &HashSet<&u32>) -> Option<Rc<Ty>> {
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

    mod apply {
        use super::*;

        #[test]
        fn empty_substitution() {
            let ty = Ty::arr(Ty::var(0), Ty::var(1));
            let s = HashMap::new();

            assert_eq!(ty, Ty::apply(&ty, &s));
        }

        #[test]
        fn type_without_variables() {
            let ty = Ty::arr(Ty::bool(), Ty::arr(Ty::nat(), Ty::bool()));

            let mut s = HashMap::new();
            s.insert(0, Ty::nat());
            s.insert(1, Ty::bool());

            assert_eq!(ty, Ty::apply(&ty, &s));
        }

        #[test]
        fn no_common_variables() {
            let ty = Ty::arr(Ty::var(0), Ty::var(1));

            let mut s = HashMap::new();
            s.insert(2, Ty::nat());
            s.insert(3, Ty::bool());

            assert_eq!(ty, Ty::apply(&ty, &s));
        }

        #[test]
        fn variable_type() {
            let ty = Ty::var(0);

            let mut s = HashMap::new();
            s.insert(0, Ty::arr(Ty::bool(), Ty::var(0)));

            assert_eq!(s[&0], Ty::apply(&ty, &s));
        }

        #[test]
        fn complex_type() {
            let ty = Ty::arr(Ty::var(0), Ty::arr(Ty::nat(), Ty::var(1)));

            let mut s = HashMap::new();
            s.insert(0, Ty::arr(Ty::var(0), Ty::bool()));
            s.insert(1, Ty::arr(Ty::nat(), Ty::arr(Ty::var(2), Ty::var(1))));
            s.insert(2, Ty::nat());

            let applied = Ty::arr(
                Ty::arr(Ty::var(0), Ty::bool()), 
                Ty::arr(
                    Ty::nat(), 
                    Ty::arr(Ty::nat(), Ty::arr(Ty::var(2), Ty::var(1)))));

            assert_eq!(applied,  Ty::apply(&ty, &s));
        }
    }
}
