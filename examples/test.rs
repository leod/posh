use fsl::{fsl, Value};

#[fsl]
fn foo(x: fsl::F32, y: fsl::F32) -> fsl::F32 {
    let z = var(x * y);
    let w = var(y + x + 1.0);

    ternary(and(z.eq(w), z.eq(1.0)), z * 2.0, 1.0)
}

#[fsl]
fn bar(x: fsl::F32) -> fsl::F32 {
    foo(x, x * 2.0) + 5.0
}

fn main() {
    let a = 2.0;

    let result = bar(a);
    //println!("{:#?}", result);

    if let fsl::lang::Expr::Call(expr) = result.expr() {
        if let fsl::lang::Func::UserDefined(func) = expr.func {
            println!("{}", fsl::lang::show::show_user_defined_funcs(&func));
        }
    }
}
