fn my_fun(x: Scalar<f32>, y: Scalar<f32>) -> Scalar<f32> {
    fn_("my_fun", {
        let_! {
            z = x / y;
            w: Scalar<f32> = y + x + 1;
        };

        z * w
    })
}

#[gloat]
fn my_fun(x: Scalar<f32>, y: Scalar<f32>) -> Scalar<f32> {
    let_! {z = x / y};
    let_! {w: Scalar<f32> = y + x + 1};

    if_(
        z < w,
        3.0,
        w + z,
    )
}

fn main() {}
