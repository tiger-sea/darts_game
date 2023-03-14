use std::io::Write;
use ansi_term::Color;


// mainからの入り口
pub fn cricket() {
    println!("\n{}", Color::Green.bold().paint("Cricket Game (If you wanna quit this game, just push enter key without any input)"));
    let mut cricket = Cricket::new();
    cricket.start_game();
}

struct Cricket {
    sections: [u8; 7], // cricket numbers
    number: [u8; 7], // how many darts in each section
    closed: [bool; 7], // whether closed or not
    point: u16, // sum of points
}

impl Cricket {
    fn new() -> Cricket {
        Cricket { sections: [20, 19, 18, 17, 16, 15, 25], number: [0; 7], closed: [false; 7], point: 0 }
    }

    // fn score_update(&mut self, place: [u16;3], num: [u16;3]) {
    //     self.point += place[0]*num[0] + place[2]*num[2] + place[2]*num[2];
    // }

    fn check_section_closed(&self, index: usize) -> bool {
        self.closed[index]
    }

    fn check_all_closed(&self) -> bool {
        self.closed.iter().all(|&x| x == true)
    }

    fn start_game(&mut self) {
        let mut round = 0;
        let mut darts = 0;

        loop {
            println!("Enter section and mark (separate with whitespace)");
            
            for j in 1..=3 {
                print!("{} -> ", j);
                std::io::stdout().flush().unwrap();
    
                // input the section and mark part
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let input: Vec<u8> = input
                                    .split_whitespace()
                                    .map(|s| s.parse().expect("Failed to parse input"))
                                    .collect();

                darts += 1;

                if input.len() == 0 { // quit the game
                    println!("{}\n", Color::Purple.paint("*** Quit the Game ***"));
                    return
                } else if input.len() != 2 || !self.sections.contains(&input[0]) || input[1] > 3 { // invalid input
                    println!("{}", Color::Yellow.paint("Invalid input"));
                    darts -= 1;
                    round -= 1;
                    break;
                } else if self.check_all_closed() { // game shot
                    let comment = format!("Game shot!🎉 {} rounds ({} darts finish)", round, darts);
                    println!("{}\n", Color::Cyan.bold().paint(comment));
                    return
                }

                let section = input[0];
                let mark = input[1];

                // TODO: 例えば2本入ってて，そこに3マーク入れると2本分は点数になることを実装する
                if self.check_section_closed(self.number[self.sections.iter().position(|&x| x == section).unwrap()] as usize) { // the section is closed
                    self.point += section as u16 * mark as u16;
                } else { // not closed yet
                    self.number[self.sections.iter().position(|&x| x == section).unwrap()] += mark;
                }
                // debug
                println!("{}, {}, {}", section, mark, self.point);
            }
            round += 1;
            println!("{}, {}", round, darts);
            self.display()
        }
    }

    fn display(&self) {
        println!("20: {}\n\
                19: {}\n\
                18: {}\n\
                17: {}\n\
                16: {}\n\
                15: {}\n\
                BULL: {}\n", 
                self.number[0], self.number[1], self.number[2], self.number[3], self.number[4], self.number[5], self.number[6]);
    }
}

/*
TODO:
20 -> |||
19 -> ||
...
みたいな感じでクリケットの表示をする
 */

// TODO: クリケットの流れ
// クリケットナンバーに3本入れるとオープン，オープンするまでは点数が入らない
// 1. ダーツを3本投げる
// 2. どこに入ったかを入力する．
// 3. オープンしているかを確認
// 4. オープンしていなければ本数を加える，オープンしていれば点数を加える
// 5. クリケットナンバーが全てオープンしたら終了
// 6. 点数とかかった本数を表示してmainへreturn

// TODO: 対戦方式でできるようにする