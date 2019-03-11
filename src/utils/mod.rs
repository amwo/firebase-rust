use std::env;

#[test]
pub fn get_home_dir() {
    match env::home_dir() {
        Some(path) => println!("{}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }
}
