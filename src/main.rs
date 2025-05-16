use rand::Rng;

fn main() {
    let mut rng = rand::rng(); // ✅ use `rng()` instead of deprecated `thread_rng()`
    let n: u32 = rng.random();    // ✅ specify the type explicitly
    println!("random number: {}", n);
}
