use posh::{
    expose::compile::compile1,
    lang::{defs::Defs, show::show_defs},
    IntoRep, Rep,
};

#[posh::def]
fn blowup(x: Rep<u32>) -> Rep<u32> {
    let b = true.into_rep();
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x = b.branch(x, x);
    let x1 = b.branch(x, x);
    let x2 = b.branch(x, x);
    x2
}

fn main() {
    let func_def = compile1(blowup).unwrap();

    //println!("{}", show_defs(&Defs::from_func_def(&func_def)));
}
