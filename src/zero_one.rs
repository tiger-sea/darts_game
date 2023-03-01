use std::io::Write;
use ansi_term::Color;
use crate::suggest;
// TODO: 点数のサジェスチョンが出る，例えば180には60,60,60とか
// TODO: 点数のサジェスチョンは一本ずつ判定する．例えば30を15Dで上がろうとして一投目に10に入ったら追加表示で10Dを表示するとか

// for detection of out of point range (arrange variation)
// const NOT_EXIST: [u16; 9] = [179, 178, 176, 175, 173, 172, 169, 166, 163];
const NOT_EXIST: [u16; 18] = [23, 25, 29, 31, 35, 37, 41, 43, 44, 46, 47, 49, 52, 53, 55, 56, 58, 59];

#[allow(unused_doc_comments)]
pub fn game(goal: u16) {
    //! for zero_one game calculation
    //! 
    //! goal: 301 or 501 or random
    /// round: basically 3 darts per round
    /// darts: the number of throw dart
    /// point: point of 1 throw

    println!("\n{} {}",
    Color::Green.bold().paint(goal.to_string()),
    Color::Green.bold().paint("Game! (If you wanna quit this game, just push enter key without any input or input non-number string)"));
    
    let mut goal = goal;
    
    let mut round = 0; // for finish round
    let mut darts = 0; // for total darts finish

    loop {
        let mut total = 0; // for 1 round points
        
        for i in 1..=3 {
            // for suggestion
            let suggest = suggest::suggest(goal, &i);
            // TODO: D25 -> BULL
            print!("{} ({}) Suggest: {:?} -> ", i, goal, suggest);
            std::io::stdout().flush().unwrap();

            // input the points
            let mut point = String::new();
            std::io::stdin()
                .read_line(&mut point)
                .expect("Point?");

            let point = match point.trim().parse() {
                Ok(point) => point,
                Err(_) => 1024,
            };

            // increment of darts throw
            darts += 1;

            // determine next situation
            if point == 1024 { // quit
                println!("{}\n", Color::Purple.paint("*** Quit the Game ***"));
                return
            } else if NOT_EXIST.contains(&point) || point > 60 { // input not existed number
                println!("{}", Color::Yellow.bold().paint("Not existed points"));
                darts -= 1;
                round -= 1;
                break
            } else if goal < point || (goal - point) == 1 { // bust
                println!("{}", Color::Yellow.bold().paint("No score..."));
                goal += total;
                total = 0;
                break
            } else if goal == point { // game shot!
                total += point;
                goal -= point;
                break
            }
            // change these for next throw
            total += point;
            goal -= point;
        }
        
        round += 1;

        if goal > 0 {
            println!("~~~ The {} Round Total: {} ~~~", round, Color::White.underline().paint(total.to_string()));
            println!("~~~ You require {} ~~~", Color::White.underline().paint(goal.to_string()));
        } else {
            let comment = format!("Game shot!🎉 {} rounds ({} darts finish)", round, darts);
            println!("{}\n", Color::Cyan.bold().paint(comment));
            break;
        }
    }
}