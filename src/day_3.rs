use std::collections::HashMap;
use std::fs::File;
use std::io::Read;


struct Sleigh { x: isize, y: isize }

impl Sleigh {
    fn new() -> Sleigh { Sleigh { x: 0, y: 0 } }

    fn next(&mut self, command : char) {
        match command {
            '^' => { self.x = self.x - 1 },
            'v' => { self.x = self.x + 1 },
            '<' => { self.y = self.y - 1 },
            '>' => { self.y = self.y + 1 },
            _ => panic!("Nope")
        };
    }
}



#[allow(unused_must_use)]
pub fn solutions() {
    let mut file = File::open("resources/santa_directions.txt").unwrap();
    let mut moves = String::new();
    file.read_to_string(&mut moves);

    let multiple_visits = part_1(&moves);
    let multiple_santa_visits = part_2(&moves);

    println!("Day 3.");
    println!("  Number of visits: {}", multiple_visits);
    println!("  Number of visits from two Santas: {}", multiple_santa_visits);

}

fn part_1(moves: &String) -> usize {
    let mut santa = Sleigh::new();
    let mut gifts = HashMap::new();

    gifts.insert((0, 0), 1);

    for c in moves.chars() {
        santa.next(c);
        update_giftmap(&santa, &mut gifts);
    }
    multiple_gifts(&gifts)
}

fn part_2(moves: &String) -> usize {
    let mut santa = Sleigh::new();
    let mut robo_santa = Sleigh::new();
    let mut gifts = HashMap::new();
    let mut santa_toggle = true;

    gifts.insert((0, 0), 2);

    for c in moves.chars() {
        let mut target_santa = match santa_toggle {
            true => &mut santa,
            false => &mut robo_santa
        };
        target_santa.next(c);

        santa_toggle = !santa_toggle;
        update_giftmap(&target_santa, &mut gifts);
    }
    multiple_gifts(&gifts)
}

fn update_giftmap(sleigh: &Sleigh, gifts: &mut HashMap<(isize, isize), usize>) {
    let key = (sleigh.x, sleigh.y);
    let count = match gifts.get(&key) { Some(v) => *v, None => 0 };
    gifts.insert(key, count + 1);
}

fn multiple_gifts(gifts: &HashMap<(isize, isize), usize>) -> usize {
    gifts.iter().filter(|kv| *kv.1 >= 1).count()
}
