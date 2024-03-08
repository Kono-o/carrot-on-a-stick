mod consts;
mod packs;
mod log;

fn main() {
    log::console(String::from(consts::CRATE_NAME));
    match packs::get_packs(){
        Some(error) => log::console(error),
        _ => log::console(String::from("all resource packs successfully retrieved!"))
    }
}