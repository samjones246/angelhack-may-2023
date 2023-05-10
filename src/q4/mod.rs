use std::{num::ParseIntError, cmp::min};

pub fn q4(input: &str) -> Result<i32, String> {
    let mut workers = parse_input(input)?;
    workers.sort();

    Ok(min_cost_with_removal(&workers))
}

fn min_cost_with_removal(workers: &Vec<i32>) -> i32 {
    let mut best: i32 = i32::MAX;
    for i in 0..workers.len() {
        let new_workers = workers[..i]
            .iter().chain(
                workers[i+1..].iter()
            )
            .collect();
        best = min(
            best,
            min_cost(&new_workers)
        )
    }
    best
}

fn min_cost(workers: &Vec<&i32>) -> i32 {
    let mut cost: i32 = 0;
    for i in (0..workers.len()).step_by(2) {
        let x = workers[i];
        let y = workers[i+1];
        cost += *y-*x;
    }
    cost
}

fn parse_input(input: &str) -> Result<Vec<i32>, String> {
    input
        .split(", ")
        .map(|e| e.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()
        .map_err(|e| e.to_string())
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_q4_example_1() {
        let input = "1, 2, 4";
        let result = q4(&input);
        assert_eq!(result, Ok(1))
    }

    #[test]
    fn test_q4_example_2() {
        let input = "4, 2, 8, 1, 9";
        let result = q4(&input);
        assert_eq!(result, Ok(2))
    }

    #[test]
    fn test_q4_example_3() {
        let input = "1, 33, 34, 50, 71, 72, 74";
        let result = q4(&input);
        assert_eq!(result, Ok(24))
    }
}