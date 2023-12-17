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

fn solution(input_str: &str) -> String {
    input_str.split(",").map(hash).sum::<usize>().to_string()
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
        assert_eq!(result, "1320");
    }
}
