// compile-flags: -Ztrait-solver=next
// check-pass

fn require_fn(_: impl Fn() -> i32) {}

fn main() {
    require_fn(|| -> i32 { 1i32 });
}
