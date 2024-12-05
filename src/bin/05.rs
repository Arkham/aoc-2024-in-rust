advent_of_code::solution!(5);

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

type Rule = (u32, u32);
type Update = Vec<u32>;
type ParsedInput = (Vec<Rule>, Vec<Update>);
type UpdateWithRules = (Update, Vec<Rule>);

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    separated_pair(
        map_res(digit1, str::parse),
        tag("|"),
        map_res(digit1, str::parse),
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(line_ending, parse_rule)(input)
}

fn parse_update(input: &str) -> IResult<&str, Update> {
    separated_list1(tag(","), map_res(digit1, str::parse))(input)
}

fn parse_input(input: &str) -> IResult<&str, ParsedInput> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, updates) = separated_list1(line_ending, parse_update)(input)?;
    Ok((input, (rules, updates)))
}

fn find_updates(input: &str) -> Option<(Vec<UpdateWithRules>, Vec<UpdateWithRules>)> {
    let (_, (rules, updates)) = parse_input(input).ok()?;

    // Use Rayon to parallelize the processing of updates
    let (valid_updates, invalid_updates): (Vec<_>, Vec<_>) = updates
        .par_iter()
        .map(|update| {
            let mut valid_rules = Vec::new();
            let mut page_map: HashMap<u32, Vec<usize>> = HashMap::new();
            for (index, &page) in update.iter().enumerate() {
                page_map.entry(page).or_default().push(index);
            }

            for rule in rules.iter() {
                if page_map.contains_key(&rule.0) && page_map.contains_key(&rule.1) {
                    valid_rules.push(*rule);
                }
            }

            let mut is_correct = true;
            for rule in valid_rules.iter() {
                let before_indices = &page_map[&rule.0];
                let after_indices = &page_map[&rule.1];
                if before_indices.iter().max() > after_indices.iter().min() {
                    is_correct = false;
                }
            }

            if is_correct {
                (Some((update.clone(), valid_rules)), None)
            } else {
                (None, Some((update.clone(), valid_rules)))
            }
        })
        .unzip();

    Some((
        valid_updates.into_iter().flatten().collect(),
        invalid_updates.into_iter().flatten().collect(),
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (valid_updates, _) = find_updates(input)?;

    let result: u32 = valid_updates
        .par_iter()
        .filter_map(|(update, _)| {
            if !update.is_empty() {
                let center_index = update.len() / 2;
                Some(update[center_index])
            } else {
                None
            }
        })
        .sum();

    Some(result)
}

fn reorder_update(update: &[u32], rules: &[Rule]) -> Vec<u32> {
    // Create a graph of dependencies
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();
    let mut nodes: HashSet<u32> = HashSet::new();

    // Build the graph and collect nodes
    for &num in update {
        nodes.insert(num);
        graph.entry(num).or_default();
        in_degree.entry(num).or_insert(0);
    }

    // Add edges based on rules
    for &(before, after) in rules {
        if nodes.contains(&before) && nodes.contains(&after) {
            graph.entry(before).or_default().push(after);
            *in_degree.entry(after).or_default() += 1;
        }
    }

    // Topological sort using Kahn's algorithm
    let mut result = Vec::new();
    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&node, _)| node)
        .collect();

    while let Some(node) = queue.pop_front() {
        result.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    // If we couldn't order all nodes, preserve original order for remaining nodes
    for &num in update {
        if !result.contains(&num) {
            result.push(num);
        }
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, invalid_updates) = find_updates(input)?;

    let result = invalid_updates
        .par_iter()
        .map(|(update, rules)| {
            let reordered = reorder_update(update, rules);
            if !reordered.is_empty() {
                let center_index = reordered.len() / 2;
                reordered[center_index]
            } else {
                0
            }
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
