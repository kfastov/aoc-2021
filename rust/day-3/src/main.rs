use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
type Result<T> = std::result::Result<T, Box <dyn Error>>;

#[derive(Debug)]
struct SubmarineParseError;
impl Error for SubmarineParseError {}
impl std::fmt::Display for SubmarineParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Failed to parse submarine instruction")
    }
}

fn get_input() -> Result<impl Iterator<Item = String>> {
    let f = File::open("./input.txt")?;
    let reader = BufReader::new(f);
    Ok(reader.lines().filter_map(|res| res.map_or(None, |x| Some(x))))
}

fn count_occurrences(strings: impl Iterator<Item = String>) -> Vec<u32> {
    strings.fold(
        None, 
        |acc, s| {
            let prev = acc
                .map_or(vec![0; s.len()], |x| x);
            Some(prev.into_iter()
                .zip(s.chars().map(|c| {
                    String::from(c).parse::<u32>().unwrap()
                }))
                .map(|(prev, last)| prev + last)
                .collect())
    }).unwrap()
}

fn bit_from_occurrence(occurrence: u32, total: usize) -> u32 {
    if occurrence < ((total + 1) / 2).try_into().unwrap() {
        0
    } else {
        1
    }
}

fn collect_binary(v: impl Iterator<Item = u32>) -> u32 {
    v.fold(0, |prev, cur| prev * 2 + cur)
}

fn parse_binary(s: String) -> u32 {
    collect_binary(s.chars().map(|c| c.to_string().parse::<u32>().unwrap()))
}

fn filter_by_bit(strings: impl Iterator<Item = String>, index: usize, value: u32) -> impl Iterator<Item = String> {
    strings.filter(move |s| s.chars().nth(index) == Some(match value {
        0 => '0',
        _ => '1',
    }))
}

fn solution1(strings: impl Iterator<Item = String>) -> u32 {
    let from_binary_vec = |vector: Vec<u32>| vector.into_iter()
        .map(|x| { if x >= 500 {1u32} else {0u32} }).reduce(|prev, cur| {prev * 2 + cur}).unwrap();
    let epsilon_from_binary_vec = |vector: Vec<u32>| vector.into_iter()
        .map(|x| { if x < 500 {1u32} else {0u32} }).reduce(|prev, cur| {prev * 2 + cur}).unwrap();

    let occurrences = count_occurrences(strings);
    let occurrences2 = occurrences.clone();
    
    let gamma = from_binary_vec(occurrences);
    let epsilon = epsilon_from_binary_vec(occurrences2);
    
    println!("{}, {}", gamma, epsilon);
    gamma * epsilon
}

#[derive(Debug)]
enum FilterPolicy {
    Most,
    Less,
}

fn filter_while(strings: &Vec<String>, index: usize, filter_policy: FilterPolicy) -> String {
    println!("Filtering by policy {:?} at index {} in {} lines:", filter_policy, &index, strings.len());
    for string in strings.iter() {
        println!("> {}", string);
    }
    let occurrences = count_occurrences(strings.clone().into_iter());
    let occurrence_at_index = occurrences[index];
    let bit_value = match filter_policy {
        FilterPolicy::Less => 1 - bit_from_occurrence(occurrence_at_index, strings.len()),
        FilterPolicy::Most => bit_from_occurrence(occurrence_at_index, strings.len()), // TODO sorry
    };
    println!("{:?} common bit at index {}: {}", filter_policy, index, bit_value);
    // filter all, if len() = 1 exit, else repeat
    let new_strings: Vec<_> = filter_by_bit(strings.clone().into_iter(), index, bit_value).collect();
    match new_strings.len() {
        1 => new_strings.first().unwrap().clone(), // TODO maybe partial destruct
        _  => {
            filter_while(&new_strings, index + 1, filter_policy)
        },
    }
}

fn solution2(strings: impl Iterator<Item = String>) -> u32 {
    let v1: Vec<_> = strings.collect();
    [FilterPolicy::Most, FilterPolicy::Less]
        .into_iter()
        .map(|policy| filter_while(&v1, 0, policy))
        .map(|s| parse_binary(s))
        .product()
}

fn main() -> Result<()> {
    println!("First star: {}", solution1(get_input()?));
    println!("Second star: {}", solution2(get_input()?));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_filter_loop() {
        let strings = vec![
            "10110",
            "11010",
            "01010",
            "10010",
            "01100",
            "11110",
            "01001",
            "10111",
            "01101",
        ].into_iter().map(String::from).collect();
        // 10111
        assert_eq!(filter_while(&strings, 0, FilterPolicy::Most), "10111".to_owned());
    }
}