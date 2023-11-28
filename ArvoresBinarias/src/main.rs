use std::io;

mod menu;

fn main() {
    loop {

        let usr_input = menu::menu::start();

        if usr_input == 0 {
            break;
        }
    }

    println!("Hello, world!");
}
