use ansi_term::Color;
use std::io::Write;

const NOT_EXIST: [u16; 18] = [23, 25, 29, 31, 35, 37, 41, 43, 44, 46, 47, 49, 52, 53, 55, 56, 58, 59];
pub fn count_up() {
    println!("\n{}", Color::Green.bold().paint("Count Up! (If you wanna quit this game, just push enter key without any input or input non-number string)"));

    let mut total: u16 = 0;
    let mut i: u8 = 1;
    loop {
        println!("The {} round", i);
        let mut round_total = 0;

        for j in 1..=3 {
            print!("{} -> ", j);
            std::io::stdout().flush().unwrap();

            // input the points part
            let mut point = String::new();
            std::io::stdin()
                .read_line(&mut point)
                .expect("Point?");

            let point: u16 = match point.trim().parse() {
                Ok(point) => point,
                Err(_) => 1024,
            };

            if point == 1024 { // quit
                println!("{}\n", Color::Purple.paint("*** Quit the Game ***"));
                return
            } else if NOT_EXIST.contains(&point) || point > 60 { // input not existed number
                println!("{}", Color::Yellow.bold().paint("Not existed points"));
                round_total = 0;
                total -= round_total;
                break
            }
            round_total += point;
        }

        total += round_total;

        if i != 8 { // continue the game
            println!("~~~ The {} Round Total: {} ~~~", i, Color::White.underline().paint(round_total.to_string()));
            println!("~~~ Total: {} ~~~", Color::White.underline().paint(total.to_string()));
            #[allow(unused_assignments)]
            if round_total == 0 { i -= 1; }
        } else { // finish the game
            let comment = format!("Finish! Total: {}", total);
            println!("{}\n", Color::Cyan.bold().paint(comment));
            return
        }

        // round increment
        i += 1;
    }
}