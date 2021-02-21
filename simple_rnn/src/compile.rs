
use egg::{*, rewrite as rw};
use crate::util;
use crate::util::{lift2};

// type Rewrite = egg::Rewrite<Math, MathAnalysis>;

define_language! {
    enum Math {
        "vec" = Vec(Vec<Id>),
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),
        "@" = As([Id; 2]),
        "'" = Prime(Id),
        Num(i32),
        Symbol(Symbol),
    }
}

#[derive(Debug, Clone, Copy)]
struct Stats {
    value: Option<i32>,
    legal: bool,
}

#[derive(Default)]
struct MathAnalysis;
impl Analysis<Math> for MathAnalysis {
    type Data = Stats;
    fn merge(&self, to: &mut Self::Data, from: Self::Data) -> bool {
        let new = Self::Data {
            value: to.value.or(from.value),
            legal: to.legal || from.legal,
        };
        egg::merge_if_different(&mut to.value, new.value) ||
        egg::merge_if_different(&mut to.legal, new.legal)
    }
    fn make(egraph: &egg::EGraph<Math, Self>, enode: &Math) -> Self::Data {
        let x = |i: &Id| egraph[*i].data;
        let v = |i: &Id| x(i).value;
        let l = |i: &Id| x(i).legal;

        let value = match enode {
            Math::Num(n) => Some(*n),
            Math::Add([a, b]) => util::lift2(|a, b| a+b, v(a), v(b)),
            Math::Mul([a, b]) => lift2(|a, b| a*b, v(a), v(b)),
            _ => None,
        };
        let legal = match enode {
            Math::Num(_) => true,
            Math::Add([a, b]) => l(a) && l(b),
            Math::Mul([a, b]) => l(a) && l(b),
            _ => false,
        };

        Self::Data { value, legal }
    }
    fn modify(egraph: &mut egg::EGraph<Math, Self>, id: Id) {
        if let Some(i) = egraph[id].data.value {
            let added = egraph.add(Math::Num(i));
            egraph.union(id, added);
        }
    }
}



#[rustfmt::skip]
fn rules() -> Vec<egg::Rewrite<Math, MathAnalysis>> { vec![
    rw!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
    rw!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
    rw!("add-0"; "(+ ?a 0)" => "?a"),
    rw!("mul-0"; "(* ?a 0)" => "0"),
    rw!("mul-1"; "(* ?a 1)" => "?a"),
    rw!("as-fst"; "(@ ?a ?b)" => "?a"),
    rw!("as-snd"; "(@ ?a ?b)" => "?b"),
    rw!("prime-dist-add"; "(+ (' ?a) (' ?b))" => "(' (+ ?a ?b))"),
    rw!("prime-dist-mul"; "(* (' ?a) (' ?b))" => "(' (* ?a ?b))"),
]}

// tests //

egg::test_fn! {
    reduce, rules(), "(+ 0 (* (+ 4 -3) foo))" => "foo",
}
egg::test_fn! { #[should_panic]
    reduce_fail, rules(), "(+ 0 (* (+ 4 -3) foo))" => "123",
}

egg::test_fn! {
    vec, rules(), "(vec (* 1 1) (* 2 2))" => "(vec 1 4)",
}
egg::test_fn! { #[should_panic]
    vec_fail, rules(), "(vec (* 1 1) (* 2 2))" => "(vec 4 1)",
}

egg::test_fn! {
    as1, rules(), "(@ foo (* 2 2))" => "foo", "4",
}
egg::test_fn! {
    as2, rules(), "(vec (@ foo (+ 1 2)) (* foo foo))" => "(vec 3 9)",
}

egg::test_fn! {
    prime, rules(), "(+ (' x) (' y))" => "(' (+ x y))",
}
egg::test_fn! { #[should_panic]
    prime_fail, rules(), "(' x)" => "x",
}



