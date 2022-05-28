use fush::{
    cond, eq, eval,
    lang::{Expr, ExprVar, Ident, Var},
    value::{func_call, Value as _},
    Scalar,
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

    func_call("my_fun", vec![var_x, var_y], args, {
        let z = eval(x * y);
        let w = eval(y + x + 1.0);

        cond(eq(z, w) * eq(z, 1.0), z * 2.0, 1.0)
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
