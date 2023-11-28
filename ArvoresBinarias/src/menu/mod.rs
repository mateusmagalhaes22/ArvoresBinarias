pub mod menu {

    use std::io;

    pub fn start() -> u8 {

        show();

        let mut usr_input_txt: String = String::new();
        io::stdin().read_line(&mut usr_input_txt).unwrap();
        return usr_input_txt.trim().parse().unwrap();
    }

    fn show() {
        println!("{:-^80}", "");
        println!("{: ^80}", "Main Menu");
        println!("{:-^80}", "");
        options();
        println!("{:-^80}", "");
    }

    fn options() {
        println!("
            0 - Exit; \n
            1 - Add on Top; \n
            2 - Remove; \n
            3 - Find; \n
            4 - Change;
        ")
    }

}