use std::{env, ops::Deref};
// use core::slice::Iter;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    
    let file_path = get_env_param(&args, "-f");
    let query = get_env_param(&args,"-q");

    println!("file_path: {file_path:?}, query: {query:?}")

}



fn get_env_param(args: &Vec<String>, key: &str) -> Option<String> {
    let mut is_last_eq_key = false;

    for item in args {
        if is_last_eq_key {
            return Some(item.to_string());
        }
        if item == key {
            is_last_eq_key = true;
        }
    };
    
    None
}