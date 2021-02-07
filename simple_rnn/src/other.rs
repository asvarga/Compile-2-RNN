

// #[derive(Debug)]
// pub struct Agent { i: Mat, s: Mat, o: Mat, m: Mat }

// impl Agent {
//     pub fn new_from_shape(si: usize, ss: usize, so: usize) -> Self {
//         Self { i: zrow(si), s: zrow(ss), o: zrow(so), m: zsqr(si+ss+so) }
//     }

//     pub fn print(&self) {
//         print!("i: "); print(&self.i);
//         print!("s: "); print(&self.s);
//         print!("o: "); print(&self.o);
//         println!("m: "); print(&self.m);
//     }

//     // def step(self, i):
//     //     self.i = sing(i)
//     //     v = np.concatenate([self.i, self.s, self.o])        # in ++ state ++ out
//     //     # TODO: ops
//     //     # print(np.ndarray.flatten(np.array([v]+[op(v) for op in ops]), 'F')) # F := column-major
//     //     v = np.ndarray.flatten(np.array([v]+[op(v) for op in ops]), 'F')
//     //     print(v)

//     //     w = np.dot(v, self.M)                               # update
//     //     self.s, self.o = w[:-self.so], w[-self.so:]         # state ++ out
//     //     return self.o

//     pub fn step(&self, i: Mat) -> Mat {
//         let v = i.clone();
//         v.extend(self.s.iter());
//         i
//     }

//     pub fn step_n(&self, n: Num) -> Mat {
//         self.step(sing(n))
//     }
// }