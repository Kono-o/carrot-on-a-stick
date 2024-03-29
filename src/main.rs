mod global;
mod packs;
mod bake;
use std::io;

fn main() {
    loop {
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
        if sel == 2 {
            let mut color_image: bake::Atlas = bake::Atlas::new(1024, bake::Maps::Color);
            color_image.bake();
            color_image.save(global::CRATE_PACKS_PATH, "color.png");
        }
        if sel > 2 { println!("invalid choice :c")}

    }

}