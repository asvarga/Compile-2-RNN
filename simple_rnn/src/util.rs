

pub fn lift2<A, B, R>(f: fn(A, B) -> R, ma: Option<A>, mb: Option<B>) -> Option<R> {
    ma.and_then(|a| mb.map(|b| f(a, b)))
}

// TODO: may need `higher`
// pub fn lift2<M, A, B, R>(f: fn(A, B) -> R, ma: M<A>, mb: M<B>) -> M<R> {
//     ma.and_then(|a| mb.map(|b| f(a, b)))
// }