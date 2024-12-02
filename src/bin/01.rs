use std::str::FromStr;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut answer: u32 = 0;
    for line in input.lines() {
        let mut values = line.split_whitespace();
        vec1.push(i32::from_str(values.next()?).unwrap_or(0));
        vec2.push(i32::from_str(values.next()?).unwrap_or(0));
//        vec2.push(values.next()?.parse::<i32>());
    }
    vec1.sort();
    vec2.sort();
    for i in 0..vec1.len() {
        answer += (vec1[i] - vec2[i]).abs() as u32;
        println!("{0}: {1} - {2} = {3} res {4}", i, vec1[i], vec2[i], (vec1[i] - vec2[i]).abs(), answer);
    }

//    println!("{0}", vec1.len());
//    println!("{0} {1}", vec1[1], vec2[1]);
//    println!("{0} {1}", vec1[1], vec2[1]);
    Some(answer)
//    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut answer: u32 = 0;
    for line in input.lines() {
        let mut values = line.split_whitespace();
        vec1.push(i32::from_str(values.next()?).unwrap_or(0));
        vec2.push(i32::from_str(values.next()?).unwrap_or(0));
//        vec2.push(values.next()?.parse::<i32>());
    }
    for x in &vec1 {
        for y in &vec2 {
            if x == y {
                answer += *x as u32;
            }
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
