pub fn pow(value: f64, power: usize) -> f64 {
    let mut powered: f64 = value;
    for _ in 0..power-1 {
        powered *= powered;
    }
    powered
}

pub fn solve_system(mut system: Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let size = system.len();

    for i in 0..size {
        for j in i..size-1 {
            if system[i][i] == 0f64 {
                continue;
            } else {
                let factor = system[j + 1][i] as f64 / system[i][i] as f64;
                for k in i..size+1 {
                    system[j + 1][k] -= factor * system[i][k] as f64;
                }
            }
        }
    }
    for i in (1..size).rev() {
        if system[i][i] == 0f64 {
            continue;
        } else {
            for j in (1..i+1).rev() {
                let factor = system[j - 1][i] as f64 / system[i][i] as f64;
                for k in (0..size+1).rev() {
                    system[j - 1][k] -= factor * system[i][k] as f64;
                }
            }
        }
    }

    let mut solutions: Vec<f64> = vec![];
    for i in 0..size {
        if system[i][i] == 0f64 {
            return None;
        }
        else {
            system[i][size] /= system[i][i] as f64;
            system[i][i] = 1f64;
            solutions.push(system[i][size])
        }
    }
    return Some(solutions);
}