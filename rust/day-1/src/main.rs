use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::iter::Iterator;

fn get_input() -> impl Iterator<Item = u32> {
    let f = File::open("./input.txt").unwrap();
    let reader = BufReader::new(f);
    reader.lines()
        .map(|result| result.map(|line| line.parse::<u32>().unwrap()))
        .map(|result| result.unwrap()) // parse
}

struct Delay<T: Copy> (Option<T>);

impl<T: Copy> Delay<T> {
    fn feed(&mut self, next: T) -> Option<T> {
        let current = self.0;
        self.0 = Some(next);
        current
    }
}

fn delay_zip_self<T: Copy>(input: impl Iterator<Item = T>) -> impl Iterator<Item = (T, T)> {
    let mut delay = Delay::<T>(None);
    input
        .map(move |a| (delay.feed(a), a))
        .skip(1)
        .map(|(a, b)| (a.unwrap(), b))
}

// fn sliding_window<T: Copy + Add<Output = T>>(input: impl Iterator<Item = T>, size: usize) -> impl Iterator<Item = T> {
//     let window = Vec::<T>::new();
//     input.filter_map(move |item| {
//         window.push(item);
//         if window.len() == size {
//             let result = window.iter().reduce(|a, b| &(*a + *b)).map(|a| *a);
//             window = window.into_iter().skip(1).collect();
//             result
//         }
//         else {
//             None
//         }
//     })
// }

fn sliding_window(input: impl Iterator<Item = u32>, size: usize) -> impl Iterator<Item = u32> {
    let mut window = Vec::<u32>::new();
    input.filter_map(move |item| {
        window.push(item);
        if window.len() == size {
            let result = window.iter().sum();
            window = window.clone().into_iter().skip(1).collect();
            Some(result)
        }
        else {
            None
        }
    })
}

fn solution_1(input: impl Iterator<Item = u32>) -> usize {
    delay_zip_self(
        input
    )
        .filter(|(a, b)| b > a)
        .count()
}

fn solution_2(input: impl Iterator<Item = u32>) -> usize {
    solution_1(sliding_window(input, 3))
}

fn main() {
    println!("First solution: {}", solution_1(get_input()));
    println!("Second solution: {}", solution_2(get_input()));
    let v = vec![1, 2, 3, 4, 5];
    assert_eq!(sliding_window(v.into_iter(), 3).collect::<Vec<_>>(), [6, 9, 12]);
}
