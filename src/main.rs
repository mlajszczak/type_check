extern crate type_check;

use std::collections::HashMap;

use type_check::Ty;

fn main() {
    let c = Ty::arr(Ty::var(0), Ty::var(1));

    let mut s = HashMap::new();
    s.insert(0, Ty::arr(Ty::var(0), Ty::bool()));
    s.insert(1, Ty::nat());

    println!("{:?}", Ty::apply(&c, &s));
}
