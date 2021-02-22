
use std::f64::INFINITY;
use std::cmp::{Ordering, max};

use egg::{*, rewrite as rw};
use crate::util::*;

// type Rewrite = egg::Rewrite<Math, MathAnalysis>;

define_language! {
    pub enum Math {
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
            Math::Add([a, b]) => lift2(|a, b| a+b, v(a), v(b)),
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

// // the derived PartialOrd will prioritize making illegal false
// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// struct ConstrainedCost {
//     illegal: bool,
//     cost: f64,
// }

#[derive(Debug, Clone, Eq)]
pub struct LinCombCost {
    depth: u32,
    is_combo: bool,
    is_mono: bool,
    is_num: bool,
}

impl Default for LinCombCost {
    fn default() -> Self {
        LinCombCost{ depth: u32::MAX, is_combo: false, is_mono: false, is_num: false, }
    }
}

impl PartialEq for LinCombCost {
    fn eq(&self, other: &Self) -> bool {
        self.is_combo && other.is_combo && self.depth == other.depth
    }
}

impl PartialOrd for LinCombCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LinCombCost {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_combo, other.is_combo) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => self.depth.cmp(&other.depth),
        }
    }
}

pub struct LinCombDepth;
impl egg::CostFunction<Math> for LinCombDepth {
    type Cost = LinCombCost;
    fn cost<C>(&mut self, enode: &Math, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        match enode {
            Math::Num(i) => LinCombCost{ depth: 1u32, 
                                              is_mono: false, 
                                              is_combo: *i == 0, 
                                              is_num: true, },
            Math::Add([x, y]) => {
                let (xcost, ycost) = (costs(*x), costs(*y));
                LinCombCost{ depth: max(xcost.depth, ycost.depth), 
                             is_combo: xcost.is_mono && ycost.is_combo, 
                             is_mono: false, 
                             is_num: false, }
            }
            Math::Mul([x, y]) => {
                let (xcost, ycost) = (costs(*x), costs(*y));
                LinCombCost{ depth: xcost.depth+1, 
                             is_combo: false, 
                             is_mono: ycost.is_num, 
                             is_num: false, }
            }
            Math::Prime(x) => {
                let xcost = costs(*x);
                LinCombCost{ depth: xcost.depth+1, 
                             is_combo: false, 
                             is_mono: false, // is this correct?
                             is_num: false, }
            }
            Math::Vec(xs) => {
                xs.iter().map(|i| costs(*i)).max().unwrap_or_else(LinCombCost::default)
            }
            _ => LinCombCost::default(),
        }
    }
}

#[rustfmt::skip]
fn rules() -> Vec<egg::Rewrite<Math, MathAnalysis>> { vec![
    rw!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
    rw!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
    rw!("assoc-add-l"; "(+ ?a (+ ?b ?c))" => "(+ (+ ?a ?b) ?c)"),
    rw!("assoc-add-r"; "(+ (+ ?a ?b) ?c)" => "(+ ?a (+ ?b ?c))"),
    rw!("add-0"; "(+ ?a 0)" => "?a"),
    rw!("mul-0"; "(* ?a 0)" => "0"),
    rw!("mul-1"; "(* ?a 1)" => "?a"),
    rw!("as-fst"; "(@ ?a ?b)" => "?a"),
    rw!("as-snd"; "(@ ?a ?b)" => "?b"),
    rw!("dist-mul-add"; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),
    rw!("dist-prime-add"; "(+ (' ?a) (' ?b))" => "(' (+ ?a ?b))"),
    rw!("dist-prime-mul"; "(* (' ?a) (' ?b))" => "(' (* ?a ?b))"),

    // we want (+ x (+ x x)) ~> (* x 3) etc. in our case we do want mul-a-rev for linear combos
    rw!("mul-1-rev"; "?a" => "(* ?a 1)"),                    // expansive
    // rw!("double"; "(+ ?a ?a)" => "(* ?a 2)"),                   // non-expansive
    // rw!("rep-add"; "(+ ?a (* ?a ?n))" => "(* (+ ?n 1) ?a)"),    // non-expansive

    // rw!("add-0-rev"; "?a" => "(+ ?a 0)"),    // FIXME: this times out :(
]}

pub fn optimize(s: &str) -> RecExpr<Math> {
    let start = s.parse().unwrap();
    let runner = Runner::default().with_expr(&start).run(&rules());
    let (egraph, root) = (runner.egraph, runner.roots[0]);
    let mut extractor = Extractor::new(&egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    return best;
}

// examples //

pub fn go(s: &str) {
    let start = s.parse().unwrap();
    let runner = Runner::default().with_expr(&start).run(&rules());
    let (egraph, root) = (runner.egraph, runner.roots[0]);
    let mut extractor = Extractor::new(&egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("{} ~~> {} <cost={}>", s, best, best_cost);    // ex: (+ 0 (* 1 10)) ~~> 10 <cost=5>
}

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

egg::test_fn! {
    rep_add_1, rules(), "(+ (+ x x) (+ x x))" => "(* x 4)",
}
egg::test_fn! {
    rep_add_2, rules(), "(+ (+ x x) (+ (+ x y) (+ (+ x y) (+ x y))))" => "(+ (* y 3) (* x 5))",
}
egg::test_fn! {
    rep_add_3, rules(), "(+ x (+ x (+ x y)))" => "(+ y (* x 3))",
}
egg::test_fn! {
    rep_add_4, rules(), "(+ (+ (+ y x) x) x)" => "(+ y (* x 3))",
}

