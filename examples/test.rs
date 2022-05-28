use fush::{
    lang::{Expr, ExprVar, Ident, Var},
    let_,
    value::{fn_, store, Scalar, Value},
};

fn my_fun(x: Scalar<f32>, y: Scalar<f32>) -> Scalar<f32> {
    let args = vec![x.expr().clone(), y.expr().clone()];
    let var_x = Var {
        ident: Ident::new("x"),
        ty: x.ty(),
    };
    let var_y = Var {
        ident: Ident::new("y"),
        ty: y.ty(),
    };
    let x = x.map_expr(|_| {
        Expr::Var(ExprVar {
            var: var_x.clone(),
            init: None,
        })
    });
    let y = y.map_expr(|_| {
        Expr::Var(ExprVar {
            var: var_y.clone(),
            init: None,
        })
    });

    fn_("my_fun", vec![var_x, var_y], args, {
        let z = store(x * y);
        let w = store(y + x + 1.0);

        z * w
    })
}

/*#[gloat]
fn my_fun(x: Scalar<f32>, y: Scalar<f32>) -> Scalar<f32> {
    let_! {z = x / y};
    let_! {w: Scalar<f32> = y + x + 1};

    if_(
        z < w,
        3.0,
        w + z,
    )
}*/

fn main() {
    let a = Scalar::from(2.0);
    let b = Scalar::from(3.0);

    let result = my_fun(a, b);
    println!("{:#?}", result);
}
