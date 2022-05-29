use fush::{
    and, branch, eval,
    lang::{Expr, ExprVar, Ident, Var},
    or,
    value::{func_call, Value as _},
    Fush,
};

fn my_fun(x: Fush<f32>, y: Fush<f32>) -> Fush<f32> {
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

        branch(and(z.eq(w), z.eq(1.0)), z * 2.0, 1.0)
    })
}

/*
#[fush]
fn my_fun2(x: Fush<f32>, y: Fush<f32>) -> Fush<f32> {
    let z = eval(x * y);
    let w = eval(y + x + 1.0);

    branch(and(z == w, z == 1.0), z * 2.0, 1.0)
}
*/

fn main() {
    let a = Fush::<f32>::from(2.0);
    let b = Fush::<f32>::from(3.0);

    let result = my_fun(a, b);
    println!("{:#?}", result);
}
