use fush::{and, fush};

#[fush]
fn my_fun(x: fush::F32, y: fush::F32) -> fush::F32 {
    let z = fush::var(x * y);
    let w = fush::var(y + x + 1.0);

    fush::branch(and(z.eq(w), z.eq(1.0)), z * 2.0, 1.0)
}

fn main() {
    let a = 2.0;
    let b = 3.0;

    let result = my_fun(a, b);
    println!("{:#?}", result);
}
