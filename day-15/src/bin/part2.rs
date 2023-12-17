use std::fmt;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("{result}");
}

fn hash(input_str: &str) -> usize {
    input_str
        .chars()
        .fold(0, |acc, ch| ((acc + (ch as u32) as usize) * 17) % 256)
}

#[derive(Debug, Clone)]
enum Operation {
    Equals(u32),
    Remove,
}

#[derive(Clone)]
struct Reading {
    label: usize,
    label_string: String,
    operation: Operation,
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "label: {}, operation: {:#?}",
            self.label_string, self.operation
        )
    }
}

impl Reading {
    fn new(input: &str) -> Reading {
        let operation_index = input
            .find(|x| x == '=' || x == '-')
            .expect("Operation not found");
        let operation = match input.chars().nth(operation_index).unwrap() {
            '=' => Operation::Equals(
                input
                    .chars()
                    .nth(operation_index + 1)
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
            ),
            '-' => Operation::Remove,
            _ => panic!("Incorrect value"),
        };
        let label_str = &input[0..operation_index];
        let label = hash(label_str);
        Reading {
            label,
            label_string: label_str.to_string(),
            operation,
        }
    }
}

fn solution(input_str: &str) -> String {
    let mut boxes: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];

    input_str
        .split(",")
        .map(|reading| Reading::new(reading))
        .for_each(|reading| {
            match reading.operation {
                Operation::Remove => {
                    if let Some(index) = boxes[reading.label]
                        .iter()
                        .position(|(label, _)| label == &reading.label_string)
                    {
                        boxes[reading.label].remove(index);
                    }
                }
                Operation::Equals(focal_length) => {
                    if let Some(index) = boxes[reading.label]
                        .iter()
                        .position(|(label, _)| label == &reading.label_string)
                    {
                        boxes[reading.label][index] = (reading.label_string, focal_length);
                    } else {
                        boxes[reading.label].push((reading.label_string, focal_length));
                    }
                }
            };
        });

    boxes
        .iter()
        .enumerate()
        .fold(0, |acc_1, (box_number, reading)| {
            acc_1
                + ((box_number + 1)
                    * reading.iter().enumerate().fold(
                        0,
                        |acc_2, (slot_number, (_, focal_length))| {
                            acc_2 + ((slot_number + 1) * *focal_length as usize)
                        },
                    ))
        })
        .to_string()
}

mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let result = hash("rn=1");
        assert_eq!(result, 30);
    }

    #[test]
    fn it_works() {
        let result = solution("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, "145");
    }
}
