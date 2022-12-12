use std::{collections::BinaryHeap};

use advent_of_code_rust::read_file;

struct Calories {
    heap: BinaryHeap<i32>
}

impl Calories {
    fn new() -> Calories {
        Calories {
            heap: BinaryHeap::new()
        }
    }

    fn sum_top_calories(mut self, input: &str, limit: i32) -> i32 {
        let mut calories = 0;

        input.lines().for_each(|line| {
            match line.parse::<i32>() {
                Ok(calorie) => calories += calorie,
                Err(_) => {
                    self.heap.push(calories);
                    calories = 0;
                }
            }
        });
        self.heap.push(calories);

        let mut sum = 0;
        for _ in 0..limit {
            sum += self.heap.pop().unwrap();
        };

        sum
    }
}

pub fn part_one(input: &str) -> i32 {
    let calories = Calories::new();
    calories.sum_top_calories(input, 1)
}

pub fn part_two(input: &str) -> i32 {
    let calories = Calories::new();
    calories.sum_top_calories(input, 3)
}

fn main() {
    let input = read_file("inputs", 1);
    println!("Solution to part 1: {}", part_one(&input));
    println!("Solution to part 2: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), 24000)
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), 45000)
    }
}