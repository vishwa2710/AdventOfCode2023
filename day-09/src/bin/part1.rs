fn main() {
    let input_str = include_str!("input.txt");
    let result = solution(input_str);
    println!("{}", result);
}

fn solution(input_str: &str) -> String {
    input_str
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i128>().unwrap())
                .collect::<Vec<i128>>()
        })
        .map(|x| {
            let mut tmp = x.clone();
            let mut last_value = tmp[tmp.len() - 1];
            while tmp.iter().any(|x| *x != 0) {
                tmp = tmp.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i128>>();
                last_value += tmp[tmp.len() - 1];
            }
            last_value
        })
        .sum::<i128>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, "114");
    }
}
