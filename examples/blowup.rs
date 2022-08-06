use posh::{
    expose::compile::compile1,
    var_form::{show::show_defs, VarFormFuncDefs},
    IntoRep, Rep,
};

#[posh::def]
fn blowup(x: Rep<u32>) -> Rep<u32> {
    let b = true.into_rep();
    let x = b.branch(x, x);
    /*let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);*/
    x
}

fn main() {
    let func_def = compile1(blowup).unwrap();

    let defs = VarFormFuncDefs::from_func_def(&func_def);
    println!("{}", show_defs(&defs));

    //println!("{}", show_defs(&Defs::from_func_def(&func_def)));
}
