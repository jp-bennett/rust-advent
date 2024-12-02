use std::str::FromStr;

advent_of_code::solution!(2);

pub fn check_values(array: &Vec<i32>) -> bool {
    let mut ascending: bool = true;
    for (i, value) in array.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if (array[i] == array[i - 1]) || (array[i] - array[i - 1]).abs() > 3 {
            return false;
        }
        if i == 1 {
            if array[i] > array[i-1] {
                ascending = true;
            } else {
                ascending = false;
            }
            continue;
        }
        if (ascending && array[i] < array[i-1]) || (!ascending && array[i] > array[i-1]) {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec1 = Vec::new();
    let mut answer: u32 = 0;
    for line in input.lines() {
        vec1.clear();
        let mut values = line.split_whitespace();
        for (i, value) in values.enumerate() {
            vec1.push(i32::from_str(value).unwrap_or(0));
        }
        if check_values(&vec1) {
            answer += 1;
        }
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec1 = Vec::new();
    let mut answer: u32 = 0;
    for line in input.lines() {
        vec1.clear();
        let mut values = line.split_whitespace();
        for (i, value) in values.enumerate() {
            vec1.push(i32::from_str(value).unwrap_or(0));
        }
        if check_values(&vec1) {
            answer += 1;
        } else {
            for i in 0..vec1.len() {
                // copy the vector
                let mut newvec = vec1.to_vec();
                newvec.remove(i);
                if check_values(&newvec) {
                    answer += 1;
                    break;
                }
            }
        }
    }
    Some(answer)
//println!("{0}", answer);

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
