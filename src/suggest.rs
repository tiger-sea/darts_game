pub fn suggest(score: u16) {
    let targets:Vec<String>;
    if score == 50 || (score <= 40 && (score % 2 == 0)) { targets = one_finish(score) }

}

// 2 throw finish
fn two_finish(score: u16) -> Vec<String> {
    let targets = vec!["stinrg".to_string()];
    targets
}

fn one_finish(score: u16) -> Vec<String> {
    // 1 throw finish
    let mut targets: Vec<String> = vec![];
    if score == 50 {
        targets.push("BULL".to_string());
    } else if score <= 40 && score % 2 == 0 {
        targets.push(format!("D{}", score / 2));
    }
    targets
}