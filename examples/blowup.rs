use posh::{
    expose::compile::compile1,
    var_form::{
        show::{show_func_defs, show_struct_defs},
        StructDefs, VarFormFuncDefs,
    },
    IntoRep, Rep,
};

#[posh::def]
fn blowup(x: Rep<u32>) -> Rep<u32> {
    let b = true.into_rep();
    let x = b.branch(x, x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 2 * x);
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
    let x = b.branch(x, 2 * x);*/
    x
}

fn main() {
    let func_def = compile1(blowup).unwrap();

    let mut structs = StructDefs::new();
    let funcs = VarFormFuncDefs::from_func_def(&func_def, &mut structs);
    println!("{}", show_struct_defs(&structs));
    println!("{}", show_func_defs(&funcs));
}
