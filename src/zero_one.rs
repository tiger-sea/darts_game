// pub fn which_one(menu: u8) {
//     match menu {
//         1 => game(301),
//         2 => game(501),
//         _ => return,
//     };
// }

use std::io::Write;
// TODO: 点数のサジェスチョンが出る，例えば180には60,60,60とか
// TODO: 点数のサジェスチョンは一本ずつ判定する．例えば30を15Dで上がろうとして一投目に10に入ったら追加表示で10Dを表示するとか

// for detection of out of point range
// const NOT_EXIST: [u16; 9] = [179, 178, 176, 175, 173, 172, 169, 166, 163];
const NOT_EXIST: [u16; 18] = [23, 25, 29, 31, 35, 37, 41, 43, 44, 46, 47, 49, 52, 53, 55, 56, 58, 59];

#[allow(unused_assignments)]
pub fn game(goal: u16) {
    println!("\n{} Game! (If you wanna quit game, just push enter without any input)", goal);

    let mut goal = goal;
    
    let mut round = 0; // for finish round
    let mut darts = 0; // for total darts finish

    loop {
        let mut total = 0; // for 1 round points
        
        for i in 1..=3 {
            print!("{} -> ", i);
            std::io::stdout().flush().unwrap();

            // input the points part
            let mut point = String::new();
            std::io::stdin()
                .read_line(&mut point)
                .expect("Point?");

            let point = match point.trim().parse() {
                Ok(point) => point,
                Err(_) => 1024,
            };

            darts += 1;

            if point == 1024 { // quit
                println!("*** Quit the Game ***\n");
                return
            } else if goal - point == 0 { // game shot
                total += point;
                goal -= point;
                break
            } else if goal < point { // burst
                println!("No score (burst)...{point}, {goal}");
                total = 0;
                break
            } else if NOT_EXIST.contains(&point) || point > 60 { // input not existed number 
                println!("Not existed points");
                darts -= 1;
                break
            }
            total += point;
            goal -= point;
        }
        
        round += 1;

        if goal > 0 {
            println!("The {} Round Total: {}", round, total);
            println!("You require {}", goal);
        } else if goal == 0 {
            println!("Game shot! {} rounds ({} darts finish)\n", round, darts);
            // unused assingment warning, but round is used
            round = 0;
            break;
        } else {
            println!("No score...");
            goal += total;
            println!("You require {}", goal);
        }
    }
}