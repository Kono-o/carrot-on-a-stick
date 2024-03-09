mod consts;
mod packs;
mod log;
mod atlas;

fn main() {
    log::console(String::from(consts::CRATE_NAME));
    if false //testing
    {
        match packs::get(){
            Some(error_code) => log::console(format!("error code [{}]", error_code)),
            _ => log::console(String::from("all resource packs successfully retrieved!"))
        }
    }
    atlas::bake();
}