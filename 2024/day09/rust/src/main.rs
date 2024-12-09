use std::fs;

#[derive(Clone)]
struct File {
    value: u64,
    index: usize,
    length: usize,
}

impl File {
    fn end_index(&self) -> usize {
        self.index + self.length
    }
}

fn filesystem_checksum(files: &Vec<File>, part1: bool) -> u64 {
    let mut files = files.clone();
    let mut i = 0;

    while i < files.len() {
        let index = files.len() - i - 1;
        let mut moved = false;

        for i in 0..index {
            let free = files[i + 1].index - files[i].end_index();

            if free >= files[index].length {
                let mut file = files.remove(index);
                file.index = files[i].end_index();
                files.insert(i + 1, file);

                moved = true;
                break;
            } else if free > 0 && part1 {
                files[index].length -= free;
                files.insert(
                    i + 1,
                    File {
                        value: files[index].value,
                        index: files[i].end_index(),
                        length: free,
                    },
                );

                moved = true;
                break;
            }
        }

        if !moved {
            if part1
                && files.iter().enumerate().skip(1).fold(0, |acc, (i, file)| {
                    acc + (file.index - files[i - 1].end_index())
                }) == 0
            {
                break;
            } else {
                i += 1;
            }
        }
    }

    files
        .iter()
        .flat_map(|file| (file.index..file.end_index()).map(|index| file.value * index as u64))
        .sum()
}

fn main() {
    let input: Vec<usize> = fs::read_to_string("../input.txt")
        .unwrap()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|val| val as usize))
        .collect();

    let mut size = 0;
    let mut files: Vec<File> = vec![];

    for (i, &digit) in input.iter().enumerate() {
        if i % 2 == 0 {
            files.push(File {
                value: (i / 2) as u64,
                index: size,
                length: digit,
            });
        }

        size += digit;
    }

    println!(
        "{}\n{}",
        filesystem_checksum(&files, true),
        filesystem_checksum(&files, false),
    );
}
