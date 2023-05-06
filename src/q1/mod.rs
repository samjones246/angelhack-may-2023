pub fn q1(input: &str) -> i32 {
    let (up, down) = count_directions(&input);
    return up - down;
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
        let result = q1("<<<");
        assert_eq!(result, 3);
    }

    #[test]
    fn q1_test_mixed() {
        let result = q1(">><<<>>");
        assert_eq!(result, -1);
    }

    #[test]
    fn q1_test_empty() {
        let result = q1("");
        assert_eq!(result, 0);
    }

    #[test]
    fn q1_test_invalid() {
        let result = q1("abcd");
        assert_eq!(result, 0);
    }
}