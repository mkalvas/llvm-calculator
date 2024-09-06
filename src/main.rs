fn main() {
    println!("{}", my_sum());
}

#[no_mangle]
fn my_sum() -> u32 {
    (0..1000).sum()
}
