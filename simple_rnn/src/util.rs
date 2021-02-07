

fn lift2<M, A, B, R>(f: fn(A, B) -> C, ma: M<A>, mb: M<B>) -> M<R> {
    ma.and_then(|a| mb.map(|b| f(a, b)))
}