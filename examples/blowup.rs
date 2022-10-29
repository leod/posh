use posh::{
    expose::compile::compile1,
    var_form::{
        show::{show_func_defs, show_struct_defs},
        StructDefs, VarFormFuncDefs,
    },
    IntoPosh, Posh,
};

#[posh::def]
fn blowup(x: Posh<u32>) -> Posh<u32> {
    let b = true.into_posh();
    let x = b.branch(x, 1 * x);
    let x = b.branch(x, 2 * x);
    let x = b.branch(x, 3 * x);
    let x = b.branch(x, 4 * x);
    let x = b.branch(x, 5 * x);
    let x = b.branch(x, 6 * x);
    let x = b.branch(x, 7 * x);
    let x = b.branch(x, 8 * x);
    let x = b.branch(x, 9 * x);
    let x = b.branch(x, 10 * x);
    let x = b.branch(x, 11 * x);
    let x = b.branch(x, 12 * x);
    let x = b.branch(x, 13 * x);
    let x = b.branch(x, 14 * x);
    let x = b.branch(x, 15 * x);
    let x = b.branch(x, 16 * x);
    let x = b.branch(x, 17 * x);
    let x = b.branch(x, 18 * x);
    let x = b.branch(x, 19 * x);
    let x = b.branch(x, 20 * x);
    let x = b.branch(x, 21 * x);
    let x = b.branch(x, 22 * x);
    let x = b.branch(x, 23 * x);
    let x = b.branch(x, 24 * x);
    let x = b.branch(x, 25 * x);
    let x = b.branch(x, 26 * x);
    let x = b.branch(x, 27 * x);
    let x = b.branch(x, 28 * x);
    let x = b.branch(x, 29 * x);
    let x = b.branch(x, 30 * x);
    let x = b.branch(x, 31 * x);
    let x = b.branch(x, 32 * x);
    x
}

fn main() {
    let func_def = compile1(blowup).unwrap();

    let mut structs = StructDefs::new();
    let funcs = VarFormFuncDefs::from_func_def(&func_def, &mut structs);
    println!("{}", show_struct_defs(&structs));
    println!("{}", show_func_defs(&funcs));
}
