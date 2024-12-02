use crate::prelude::*;
use std::collections::HashMap;
use std::iter::zip;

pub struct Day01;

type DataType = i32;

impl Puzzle for Day01 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, _input: &str) -> super::PuzzleResult {
        // dbg!(_input);
        let (mut l1, mut l2): (Vec<_>, Vec<_>) = _input
            .trim()
            .lines()
            .map(|l| {
                // dbg!(l);
                let chunks = l.split_whitespace().map(|n_str| {
                    n_str
                        .parse::<DataType>()
                        .expect("Failed to unwrap data value")
                });
                // println!("{:?}", chunks.clone().collect_vec());
                return chunks
                    .collect_tuple::<(DataType, DataType)>()
                    .expect("Wrong number of data-values");
            })
            .unzip();

        l1.sort();
        l2.sort();

        return Ok(zip(l1, l2)
            .map(|(a, b)| (a - b).abs())
            .sum::<DataType>()
            .to_string());
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        let (left, right): (Vec<_>, Vec<_>) = _input
            .trim()
            .lines()
            .map(|l| {
                // dbg!(l);
                let chunks = l.split_whitespace().map(|n_str| {
                    n_str
                        .parse::<DataType>()
                        .expect("Failed to unwrap data value")
                });
                // println!("{:?}", chunks.clone().collect_vec());
                return chunks
                    .collect_tuple::<(DataType, DataType)>()
                    .expect("Wrong number of data-values");
            })
            .unzip();

        let mut freq_sum: HashMap<DataType, i32> = HashMap::new();
        for r in right {
            let ent = freq_sum.entry(r).or_default();
            // "multiply the number in the left list `r` by number of times seen in the right"
            *ent += r;
        }
        let result = left
            .iter()
            .map(|l| freq_sum.entry(l.clone()).or_default().clone())
            .sum::<i32>();
        return Ok(result.to_string());
    }
}

impl Day01 {}

#[test]
fn sample_1() {
    let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;
    println!("{:?}", Day01.part_one(input).unwrap());
}
