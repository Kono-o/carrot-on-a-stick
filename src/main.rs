mod global;
mod packs;
mod bake;
use std::io;

fn main() {
    println!("{}", global::CRATE_NAME);

    println!("1> packs 2> atlas");
    let mut sel = String::new();
    io::stdin()
        .read_line(&mut sel)
        .expect("io error");

    let sel: u32 = sel.trim().parse().unwrap();
    if sel == 1 {
        let code = packs::get_from_mc_folder();
        match code{
            0 => println!("packs-> success!"),
            _ => println!("packs-> error[{}]!",code)
        }
    }
    else { bake::bake(); }

}