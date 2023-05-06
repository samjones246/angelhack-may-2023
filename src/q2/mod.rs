use std::cmp::min;
use std::num::ParseIntError;

pub fn q2(input: &str, race_time: i32) -> Result<i32, String> {
    let runners = parse_runners(input)?;
    let result = runners
        .iter()
        .map(|r| distance_covered_in_time(r, race_time))
        .max();
    match result {
        Some(x) => Ok(x),
        None => Err(String::from("There were no runners"))
    }
}

fn parse_runners(input: &str) -> Result<Vec<Runner>, String> {
    input.lines().map(Runner::build).collect()
}

struct Runner {
    speed: i32,
    run_time: i32,
    rest_time: i32,
}

impl Runner {
    fn cycle(&self) -> (i32, i32) {
        let length = self.run_time + self.rest_time;
        let distance = self.run_time * self.speed;
        (length, distance)
    }

    fn build(input: &str) -> Result<Runner, String> {
        let parts: Result<Vec<i32>, ParseIntError> = input
            .split(",")
            .map(|n| n.parse())
            .collect();
        let parts = parts
            .map_err(|_| "Could not interpret all descriptor compoenents as integers")?;
        if parts.len() != 3 {
            return Err(String::from("Descriptor must contain exactly 3 components"));
        };
        Ok(Runner {
            speed: parts[0],
            run_time: parts[1],
            rest_time: parts[2]
        })
    }
}

fn distance_covered_in_time(runner: &Runner, time: i32) -> i32 {
    let (cycle_len, cycle_dist) = runner.cycle();
    let cycles_done = time / cycle_len;
    let subtotal = cycle_dist * cycles_done;
    let time_remaining = time - (cycles_done * cycle_len);
    let time_remaining = min(time_remaining, runner.run_time);
    subtotal + (time_remaining * runner.speed)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance_covered_in_time_one_cycle_running() {
        let runner = Runner {
            speed: 10,
            run_time: 6,
            rest_time: 20
        };
        let result = distance_covered_in_time(&runner, 4);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_distance_covered_in_time_one_cycle_resting() {
        let runner = Runner {
            speed: 10,
            run_time: 6,
            rest_time: 20
        };
        let result = distance_covered_in_time(&runner, 8);
        assert_eq!(result, 60);
    }

    #[test]
    fn test_distance_covered_in_time_many_cycles_running() {
        let runner = Runner {
            speed: 10,
            run_time: 6,
            rest_time: 20
        };
        let result = distance_covered_in_time(&runner, 28);
        assert_eq!(result, 80);
    }

    #[test]
    fn test_distance_covered_in_time_many_cycles_resting() {
        let runner = Runner {
            speed: 10,
            run_time: 6,
            rest_time: 20
        };
        let result = distance_covered_in_time(&runner, 33);
        assert_eq!(result, 120);
    }

    #[test]
    fn test_q2_simple() {
        let input = "10,6,20\n8,8,25";
        let race_time = 100;
        let result = q2(input, race_time);
        assert_eq!(result, Ok(240));
    }
}