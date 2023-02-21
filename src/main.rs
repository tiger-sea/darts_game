use std::io;
use std::io::Write;
use ansi_term::Color;

// my functions
mod zero_one;
mod cricket;
mod random;


fn main() -> Result<(), ()> {
    println!("");
    loop {
        println!("{}", Color::Green.bold().paint("Let's play darts!🎯"));
        print!("1: 301 Game\n\
                2: 501 Game\n\
                3: Cricket Game\n\
                4: Finish Practice\n\
                0: Exit\n\
                Choose a menu! -> ");
        // print!("Choose a menu! [1: 301 Game, 2: 501 Game, 3: Cricket Game, 4: Finish Practicek 0: Exit] -> ");
        std::io::stdout().flush().unwrap();
        
        // input part of practice menu
        let mut menu = String::new();
        io::stdin()
            .read_line(&mut menu)
            .expect("Failed to read line");

        let menu: u8 = match menu.trim().parse() {
            Ok(menu) => menu,
            Err(_) => 255,
        };

        // assign a menu
        match menu {
            1 => zero_one::game(301),
            2 => zero_one::game(501),
            3 => cricket::cricket(),
            4 => random::random(),
            0 => {
                println!("\n{}\n", Color::Green.bold().paint("+++ Thank you! +++"));
                return Ok(())
            },
            _ => {
                println!("{}", Color::Purple.paint("\n*** Sorry, it is invalid input... ***\n"));
                continue
            },
        };
    }
}

// TODO: input!マクロで書き直す