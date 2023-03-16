// finish combination
pub fn suggest(score: u16, left_darts: &u8) -> Vec<String> {
    let mut targets = vec![];
    let left_darts = 3 - *left_darts;

    // 1 throw finish
    if score <= 40 && score % 2 == 0 {
        targets.push(format!("D{}", score / 2));
        return targets;
    } else if left_darts == 0 && score == 50 {
        targets.push(format!("BULL"));
        return targets
    }

    let nums: [u16; 20] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];
    let mut found = false;
    let mut targets = vec![];
    for i in (0..nums.len()).rev() {
        for j in (0..nums.len()).rev() {
            if score == (nums[i] + nums[j] * 2) { // 1. single, 2. double *highest priority
                // let mut targets = vec![];
                targets.push(format!("S{}, D{}", nums[i], nums[j]));
                found = true;
                // return targets
            } else if score == (nums[i] * 2 + nums[j] * 2) { // 1. double, 2. double
                targets.push(format!("D{} D{}", nums[i], nums[j]));
                found = true;
            } else if score == (nums[i] * 3 + nums[j] * 2) { // 1. triple, 2. double
                targets.push(format!("T{} D{}", nums[i], nums[j]));
                found = true;
            }

            if !found {
                for k in (0..nums.len()).rev() {
                    if score == (nums[i] * 3 + nums[j] + nums[k] * 2) { // 1. triple, 2. single, 3. double
                        targets.push(format!("T{} S{}, D{}", nums[i], nums[j], nums[k]));
                        found = true;
                    } else if score == (nums[i] * 3 + nums[j] * 3 + nums[k] * 2) { // 1. triple, 2. triple, 3. double
                        targets.push(format!("T{}, T{}, D{}", nums[i], nums[j], nums[k]));
                        found = true;
                    } else if score == (nums[i] * 3 + nums[j] * 2 + nums[k] * 2) {
                        targets.push(format!("T{}, D{}, D{}", nums[i], nums[j], nums[k])); // 1. triple, 2. double, 3. double
                        found = true;
                    }
                }
                
            }
        }
    }
    targets
}
