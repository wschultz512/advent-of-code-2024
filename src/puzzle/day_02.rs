use crate::prelude::*;

pub struct Day02;

type DataType = i32;

impl Puzzle for Day02 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, _input: &str) -> super::PuzzleResult {
        let reports = _input.trim().lines().map(|l| {
            // dbg!(l);
            let chunks = l.split_whitespace().map(|n_str| {
                n_str
                    .parse::<DataType>()
                    .expect("Failed to unwrap data value")
            });
            // println!("{:?}", chunks.clone().collect_vec());
            return chunks;
        });
        let safety_scores = reports.map(report_is_safe).filter(|s| s.clone());
        let count = safety_scores.count();
        return Ok(count.to_string());
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        let reports = _input.trim().lines().map(|l| {
            // dbg!(l);
            let chunks = l.split_whitespace().map(|n_str| {
                n_str
                    .parse::<DataType>()
                    .expect("Failed to unwrap data value")
            });
            // println!("{:?}", chunks.clone().collect_vec());
            return chunks;
        });
        let safety_scores = reports
            .map(|iter| {
                let values = iter.collect_vec();
                if report_is_safe(values.iter().copied()) {
                    return true;
                }
                return (0..values.len())
                    .map(|idx| skipper_iter(0, idx, values.iter().copied()))
                    .any(report_is_safe);
            })
            .filter(|s| s.clone());
        let count = safety_scores.count();
        return Ok(count.to_string());
    }
}

struct SkippingIndexIterator<T, Itor>
where
    Itor: Iterator<Item = T>,
{
    index: usize,
    skip_target: usize,
    iter: Itor,
}
impl<T, I> Iterator for SkippingIndexIterator<T, I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = self.iter.next()?;
        if self.index == self.skip_target {
            value = self.iter.next()?;
            self.index += 1;
        }
        self.index += 1;
        Some(value)
    }
}
fn skipper_iter<T, I>(current_index: usize, skip: usize, iter: I) -> SkippingIndexIterator<T, I>
where
    I: Iterator<Item = T>,
{
    SkippingIndexIterator {
        index: current_index,
        skip_target: skip,
        iter,
    }
}

fn report_is_safe(iter: impl Iterator<Item = DataType>) -> bool {
    let mut first: DataType = 0;
    let mut second: DataType;
    let mut ascending = false;
    let mut last: DataType = 0;
    for (idx, val) in iter.enumerate() {
        match idx {
            0 => {
                first = val;
                continue;
            }
            1 => {
                second = val;
                last = first;
                ascending = first < second;
            }
            _ => {}
        }

        if ascending != (last < val) {
            return false;
        }
        let still_safe = (1..=3).contains(&last.abs_diff(val));
        if !still_safe {
            return false;
        }
        last = val
    }
    return true;
}

#[test]
fn sample_02_1() {
    let input = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    println!("{:?}", Day02.part_one(input).unwrap());
}
#[test]
fn sample_02_2() {
    let input = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    println!("{:?}", Day02.part_two(input).unwrap());
}
