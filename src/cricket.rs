use std::io::Write;
use ansi_term::Color;


// mainからの入り口
pub fn cricket() {
    println!("\n{}", Color::Green.bold().paint("Cricket Game (If you wanna quit this game, just push enter key without any input)"));
    let mut cricket = Cricket::new();
    cricket.start_game();
}

#[derive(Debug)]
struct Cricket {
    sections: [u8; 7], // cricket numbers and 0
    number: [u8; 7], // how many darts in each section
    closed: [bool; 7], // whether closed or not
    point: u16, // sum of points
}

impl Cricket {
    fn new() -> Cricket {
        Cricket { sections: [20, 19, 18, 17, 16, 15, 25], number: [0; 7], closed: [false; 7], point: 0 }
    }

    fn score_update(&mut self, section: u8, mark: u8, index: usize) {

        if mark == 0 { return } // no mark

        if (mark + self.number[index]) > 3 {
            let mark = mark + self.number[index] - 3;
            self.number[index] = 3;
            self.point += (section * mark) as u16; // upper limit is 60 at once so type transformation is okay
            self.closed[index] = true;
        } else {
            self.number[index] += mark;
        }
        self.closed_update(index);
    }

    fn closed_update(&mut self, index: usize) {
        if self.number[index] == 3 {
            self.closed[index] = true;
        }
    }

    #[allow(dead_code)]
    fn check_section_closed(&self, index: usize) -> bool {
        self.closed[index]
    }

    fn check_all_closed(&self) -> bool {
        self.closed.iter().all(|&x| x == true)
    }

    fn start_game(&mut self) {
        let mut round = 0;
        let mut darts = 0;
        let mut marks = 0;

        loop {
            println!("Enter section and number of marks (separate with whitespace)");
            
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

                // throw darts increment
                darts += 1;

                if input.len() == 0 { // quit the game
                    println!("{}\n", Color::Purple.paint("*** Quit the Game ***"));
                    return
                } else if input[0] == 0 {
                    continue;
                } else if input.len() != 2 || !self.sections.contains(&input[0]) || input[1] > 3 { // invalid input
                    println!("{}", Color::Yellow.paint("Invalid input"));
                    darts -= 1;
                    round -= 1;
                    break;
                }

                // devide input into two
                let section = input[0];
                let mark = input[1];
                marks += mark;

                // index of input section
                let index = self.sections.iter().position(|&x| x == section).unwrap();

                // update number and points
                self.score_update(section, mark, index);

                if self.check_all_closed() { // game shot
                    round += 1;
                    let comment = format!("Game shot!🎉 {} rounds ({} darts finish)", round, darts);
                    let stats = format!("Agerage: {} marks per round", marks/round);
                    println!("{}", Color::Cyan.bold().paint(comment));
                    println!("{}\n", Color::Cyan.bold().paint(stats));
                    return
                }
            }
            round += 1;
            self.display()
        }
    }

    fn display(&self) {
        let mut symbols: Vec<String> = Vec::new();
        let iter = self.number.iter();
        let label_iter = ["20", "19", "18", "17", "16", "15", "BULL"].iter();
        for (num, label) in iter.zip(label_iter) {
            let symbol = match num {
                3 => "|||",
                2 => "||",
                1 => "|",
                _ => "",
            };
            symbols.push(format!("{}: {}", label, symbol));
        }

        println!(
            "\n{}\n\
            {}\n\
            Points: {}\n",
            Color::White.bold().underline().paint("Progress"),
            symbols.join("\n"),
            self.point
        );
    }
}

// TODO: クリケットの流れ
// クリケットナンバーに3本入れるとオープン，オープンするまでは点数が入らない
// 1. ダーツを3本投げる
// 2. どこに入ったかを入力する．
// 3. オープンしているかを確認
// 4. オープンしていなければ本数を加える，オープンしていれば点数を加える
// 5. クリケットナンバーが全てオープンしたら終了
// 6. 点数とかかった本数を表示してmainへreturn

// TODO: 対戦方式でできるようにする

// TODO: 平均マーク数を計算する