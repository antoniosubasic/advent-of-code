use std::{collections::HashMap, fs};

fn main() {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in fs::read_to_string("../input.txt").unwrap().lines() {
        let mut nums = line.split("   ").map(|num| num.parse::<i32>().unwrap());

        list1.push(nums.next().unwrap());
        list2.push(nums.next().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut distance = 0;

    let mut similarities: HashMap<i32, i32> = HashMap::new();
    let mut similarity_score = 0;

    for i in 0..list1.len() {
        distance += (list1[i] - list2[i]).abs();

        if let Some(score) = similarities.get(&list1[i]) {
            similarity_score += score;
        } else {
            let score = list1[i] * list2.iter().filter(|&&num| num == list1[i]).count() as i32;
            similarities.insert(list1[i], score);
            similarity_score += score;
        }
    }

    println!("{distance}\n{similarity_score}");
}
