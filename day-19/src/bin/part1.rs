use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

fn main() {
    let result = solution(include_str!("input.txt"));
    println!("{}", result);
}

#[derive(Debug)]
enum Operator {
    LessThan,
    GreaterThan,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::LessThan => write!(f, "<"),
            Operator::GreaterThan => write!(f, ">"),
        }
    }
}

#[derive(Debug)]
struct Rule {
    variable: Variable,
    operator: Operator,
    next: String,
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}:{}",
            self.variable.name, self.operator, self.variable.value, self.next
        )
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    else_case: String,
}

impl Display for Workflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name,
            self.rules
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Variable {
    name: String,
    value: u32,
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl Ord for Variable {
    fn cmp(&self, other: &Self) -> Ordering {
        let name_order = self.name.cmp(&other.name);
        if name_order != Ordering::Equal {
            name_order
        } else {
            self.value.cmp(&other.value)
        }
    }
}

impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_line(variables: &Vec<Variable>, workflows: &HashMap<String, Workflow>) -> u32 {
    let mut current_workflow = "in".to_string();

    println!(
        "INPUT WITH: {}",
        &variables
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    while current_workflow != "R" && current_workflow != "A" {
        current_workflow = get_next_workflow(current_workflow, &variables, &workflows);
    }
    println!("\n\n");

    if current_workflow == "R" {
        0
    } else {
        variables.iter().fold(0, |acc, v| acc + v.value)
    }
}

fn get_next_workflow(
    current_workflow: String,
    variables: &Vec<Variable>,
    workflows: &HashMap<String, Workflow>,
) -> String {
    let workflow = workflows
        .get(&current_workflow)
        .expect(format!("Workflow {} not found", current_workflow).as_str());

    for rule in workflow.rules.iter() {
        if let Some(variable) = variables.iter().find(|v| v.name == rule.variable.name) {
            match rule.operator {
                Operator::LessThan => {
                    if variable < &rule.variable {
                        println!("Workflow [{}]", workflow);
                        return rule.next.clone();
                    }
                }
                Operator::GreaterThan => {
                    if variable > &rule.variable {
                        println!("Workflow [{}]", workflow);
                        return rule.next.clone();
                    }
                }
            }
        }
    }

    println!(
        "Workflow [{}] else condition -> {}",
        workflow, workflow.else_case
    );

    return workflow.else_case.clone();
}

fn parse_workflows(line: &str) -> Workflow {
    let re = Regex::new(r"(\w+)\{(.+),(\w+)\}").unwrap();
    let re_rule = Regex::new(r"(\w+)([<>])(\d+):(\w+)").unwrap();

    if let Some(caps) = re.captures(line) {
        let main_name = &caps[1];
        let rules = &caps[2];
        let else_case = &caps[3];

        let mut rules_map = Vec::new();

        for rule in rules.split(',') {
            if let Some(caps) = re_rule.captures(rule) {
                let variable_name = &caps[1];
                let operator = &caps[2];
                let value = &caps[3];
                let workflow_name = &caps[4];
                let rule = Rule {
                    variable: Variable {
                        name: variable_name.to_string(),
                        value: value.parse::<u32>().unwrap(),
                    },
                    operator: match operator {
                        "<" => Operator::LessThan,
                        ">" => Operator::GreaterThan,
                        _ => panic!("Invalid operator"),
                    },
                    next: workflow_name.to_string(),
                };
                rules_map.push(rule);
            }
        }

        return Workflow {
            name: main_name.to_string(),
            rules: rules_map,
            else_case: else_case.to_string(),
        };
    }

    panic!("Cannot parse");
}

fn parse_ratings(input_str: &str) -> Vec<Variable> {
    input_str[1..input_str.len() - 1]
        .split(",")
        .map(|pair| {
            let mut pair = pair.split("=");
            let name = pair.next().unwrap();
            let value = pair.next().unwrap().parse::<u32>().unwrap();
            Variable {
                name: name.to_string(),
                value,
            }
        })
        .collect::<Vec<Variable>>()
}

fn solution(input_str: &str) -> String {
    let data = input_str.split("\n\n").collect::<Vec<&str>>();

    let workflows = data[0]
        .lines()
        .map(parse_workflows)
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect::<HashMap<String, Workflow>>();

    let inputs = data[1]
        .lines()
        .map(parse_ratings)
        .collect::<Vec<Vec<Variable>>>();

    inputs
        .iter()
        .fold(0, |acc, input| acc + solve_line(input, &workflows))
        .to_string()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result, "19114");
    }
}
