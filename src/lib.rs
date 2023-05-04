use std::io::Error;

pub const QUESTIONS_IMPLEMENTED: i32 = 1;

pub fn q1(input: &str) -> Result<i32, Error> {
    let (up, down) = count_directions(&input);
    return Ok(up - down);
}

fn count_directions(input: &str) -> (i32, i32) {
    let up = input
        .chars()
        .filter(|c| *c == '<')
        .count() as i32;
    let down = input
        .chars()
        .filter(|c| *c == '>')
        .count() as i32;
    (up, down)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn q1_test_only_up() {
        let result = q1("<<<").unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn q1_test_mixed() {
        let result = q1(">><<<>>").unwrap();
        assert_eq!(result, -1);
    }
}
