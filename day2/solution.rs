use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut levels = Vec::with_capacity(20);
    let mut modified = Vec::with_capacity(20);

    let mut safe_count_without_dampener = 0;
    let mut safe_count_with_dampener = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        levels.clear();
        levels.extend(
            line.split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok())
        );

        let is_safe_original = is_safe(&levels);
        safe_count_without_dampener += is_safe_original as usize;

        if is_safe_original {
            safe_count_with_dampener += 1;
            continue;
        }

        let is_safe_with_dampener = (0..levels.len()).any(|i| {
            modified.clear();
            modified.extend(
                levels
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &x)| {
                        if j != i {
                            Some(x)
                        } else {
                            None
                        }
                    })
            );
            is_safe(&modified)
        });

        safe_count_with_dampener += is_safe_with_dampener as usize;
    }

    println!("no damper: {}", safe_count_without_dampener);
    println!("damper: {}", safe_count_with_dampener);
    Ok(())
}

fn is_safe(levels: &[i32]) -> bool {
    let len = levels.len();
    if len < 2 {
        return true;
    }

    let mut direction = 0; // 0 = unknown, 1 = increasing, -1 = decreasing
    let mut prev = levels[0];

    for &current in &levels[1..] {
        let diff = current - prev;
        
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        match direction {
            0 => direction = diff.signum(),
            d if d != diff.signum() => return false,
            _ => {}
        }

        prev = current;
    }

    true
}
