use std::fs;

struct Coordinate<T> {
    x: T,
    y: T,
}

impl<T> Coordinate<T> {
    fn parse(s: &str) -> Result<Self, T::Err>
    where
        T: std::str::FromStr,
    {
        let mut parts = s.split_once('=').unwrap().1.split(',');

        Ok(Self {
            x: parts.next().unwrap().parse()?,
            y: parts.next().unwrap().parse()?,
        })
    }
}

struct Robot {
    position: Coordinate<usize>,
    velocity: Coordinate<i32>,
}

impl Robot {
    fn move_robot(&mut self, space: &Coordinate<usize>) {
        let new_x = {
            let delta = self.velocity.x.abs() as usize;

            if self.velocity.x.is_negative() {
                if delta > self.position.x {
                    space.x - delta + self.position.x
                } else {
                    self.position.x - delta
                }
            } else {
                (self.position.x + delta) % space.x
            }
        };

        let new_y = {
            let delta = self.velocity.y.abs() as usize;

            if self.velocity.y.is_negative() {
                if delta > self.position.y {
                    space.y - delta + self.position.y
                } else {
                    self.position.y - delta
                }
            } else {
                (self.position.y + delta) % space.y
            }
        };

        self.position.x = new_x;
        self.position.y = new_y;
    }
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut input: Vec<Robot> = input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();

            Robot {
                position: Coordinate::parse(p).unwrap(),
                velocity: Coordinate::parse(v).unwrap(),
            }
        })
        .collect();

    let space = Coordinate { x: 101, y: 103 };

    for _ in 0..100 {
        for robot in input.iter_mut() {
            robot.move_robot(&space);
        }
    }

    let mid = Coordinate {
        x: space.x / 2,
        y: space.y / 2,
    };

    let mut quadrants = vec![vec![0; 2]; 2];
    for robot in &input {
        if robot.position.y != mid.y && robot.position.x != mid.x {
            quadrants[if robot.position.y < mid.y { 0 } else { 1 }]
                [if robot.position.x < mid.x { 0 } else { 1 }] += 1;
        }
    }

    println!(
        "{}",
        quadrants
            .into_iter()
            .flatten()
            .reduce(|acc, element| acc * element)
            .unwrap()
    );
}
