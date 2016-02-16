#![feature(unboxed_closures, fn_traits)]

fn succ(x: i32) -> i32 { x + 1 }
fn sq(x: i32) -> i32 { x * x }

struct Compose<F, G>(F, G); 
impl<R, M, A, F: FnOnce(M) -> R, G: FnOnce<A, Output=M>> FnOnce<A> for Compose<F, G> {
    type Output = R;
    extern "rust-call" fn call_once(self, args: A) -> R {
        let Compose(f, g) = self;
        f(g.call_once(args))
    }
}
impl<R, M, A, F: FnMut(M) -> R, G: FnMut<A, Output=M>> FnMut<A> for Compose<F, G> {
    extern "rust-call" fn call_mut(&mut self, args: A) -> R {
        self.0(self.1.call_mut(args))
    }
}
impl<R, M, A, F: Fn(M) -> R, G: Fn<A, Output=M>> Fn<A> for Compose<F, G> {
    extern "rust-call" fn call(&self, args: A) -> R {
        self.0(self.1.call(args))
    }
}

fn main() {
    let f = Compose(succ, sq);
    println!("{} {}", f(5), f(6));
}
