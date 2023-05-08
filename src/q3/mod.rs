use itertools::Itertools;

pub fn q3(input: i32) -> i32 {
    let perms = digit_permutations(input);
    let mut valid: Vec<&u32> = perms.iter()
        .filter(|x| *x % 7 == 0)
        .collect();
    valid.sort();
    if valid.len() == 0 {
        0
    } else {
        let min = valid.first().unwrap();
        let max = valid.last().unwrap();
        let res = (*max + *min) / 2;
        res.try_into().unwrap()
    }
}

fn digit_permutations(x: i32) -> Vec<u32> {
    let digits: Vec<u32>  = x.to_string()
        .chars()
        .map(|d|d.to_digit(10).unwrap())
        .collect();

    digits
        .iter()
        .permutations(digits.len())
        .map(concat_digits)
        .collect()
}

fn concat_digits(digits: Vec<&u32>) -> u32 {
    digits.iter()
        .fold(0u32, |acc, elem| acc * 10 + *elem)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_q3_base() {
        let result = q3(789);
        assert_eq!(result, 892);
    }
}