use std::collections::HashMap;

pub fn q5(input: &str) -> String {
    let (encoded, codebook) = parse_input(input);
    decode(encoded, codebook)
}

fn parse_input(input: &str) -> (&str, HashMap<&str, char>) {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() < 2 {
        panic!("Not enough lines in input");
    }
    let entries = lines[0].split(",");
    let mut codebook: HashMap<&str, char> = HashMap::new();
    for e in entries {
        let entry: Vec<&str> = e.split(":").collect();
        let v = entry[0].chars().collect::<Vec<char>>();
        let v = v.first().unwrap();
        let k = entry[1];
        codebook.insert(k, *v);
    }
    let encoded = lines[1];
    (encoded, codebook)
}

fn decode(encoded: &str, codebook: HashMap<&str, char>) -> String {
    let codebook_tree = get_codebook_tree(codebook);
    let mut out: Vec<char> = Vec::new();
    let mut current_node = &codebook_tree;
    for bit in encoded.chars() {
        current_node = match bit {
            '0' => current_node.left.as_ref().unwrap(),
            '1' => current_node.right.as_ref().unwrap(),
            _ => panic!("Invalid symbol in encoded message")
        };
        if let Some(c) = current_node.value {
            current_node = &codebook_tree;
            out.push(c);
        }
    }
    return String::from_iter(out);
}

#[derive(Debug)]
struct Node {
    value: Option<char>,
    level: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    fn new(level: usize) -> Node {
        Node {
            value: None,
            level: level,
            left: None,
            right: None
        }
    }

    fn push(&mut self, key: String, value: char) {
        if key.len() == self.level {
            self.value = Some(value);
            return;
        }
        let chars: Vec<char> = key.chars().collect();
        let next_bit: &char = chars
            .get(self.level)
            .unwrap();
        
        let next_node = match next_bit {
            '0' => {
                if self.left.is_none() {
                    self.left = Some(
                        Box::new(
                            Node::new(self.level + 1)
                        )
                    );
                }
                self.left.as_mut().unwrap()
            },
            '1' => {
                if self.right.is_none() {
                    self.right = Some(
                        Box::new(
                            Node::new(self.level + 1)
                        )
                    );
                }
                self.right.as_mut().unwrap()
            },
            _ => panic!("Invalid symbol in codebook key")
        };
        next_node.push(key, value);
    }
}

fn get_codebook_tree(codebook: HashMap<&str, char>) -> Node {
    let mut root = Node::new(0);
    for (k, v) in codebook.iter() {
        root.push(String::from(*k), *v)
    }
    root
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_codebook_tree() {
        let mut codebook = HashMap::new();
        codebook.insert("00", 'a');
        codebook.insert("01", 'b');
        codebook.insert("11", 'c');
        let root = get_codebook_tree(codebook);
        let a = root
            .left
            .as_ref()
            .unwrap()
            .left
            .as_ref()
            .unwrap();
        let b = root
            .left
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap();
        let c = root
            .right
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap();
        assert_eq!(a.value, Some('a'));
        assert_eq!(b.value, Some('a'));
        assert_eq!(c.value, Some('a'));
    }
}