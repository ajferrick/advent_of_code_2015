use std::cmp::min as min;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;


struct GiftBox { l: usize, w: usize, h: usize }

impl GiftBox {
    fn from_str(line: String) -> GiftBox {
        let dim_string = line.split("x");
        let dims : Vec<usize> = dim_string.map(|s| usize::from_str(s).unwrap()).collect();

        let l = match dims.get(0) { Some(l) => *l, None => panic!("Not here!") };
        let w = match dims.get(1) { Some(w) => *w, None => panic!("Not here!") };
        let h = match dims.get(2) { Some(h) => *h, None => panic!("Not here!") };

        GiftBox { l: l, w: w, h: h }
    }

    fn volume(&self) -> usize {
        self.l * self.w * self.h
    }

    fn surface_area(&self) -> usize {
        let top = self.l * self.w;
        let front = self.w * self.h;
        let side = self.l * self.h; 
        let slack = min(min(top, front), side);
        2 * top + 2 * side + 2 * front + slack
    }

    fn smallest_perim(&self) -> usize {
        let p = 2 * self.l + 2 * self.h;
        let r = 2 * self.h + 2 * self.w;
        let y = 2 * self.w + 2 * self.l;
        min(min(p, r), y)
    }

    fn ribbon(&self) -> usize {
        self.smallest_perim() + self.volume()
    }
}


pub fn solutions() {
    let file = File::open("resources/box_dimensions.txt").unwrap();
    let gifts : Vec<GiftBox> = BufReader::new(file).lines()
        .map(|l| l.unwrap())
        .map(|l| GiftBox::from_str(l))
        .collect();

    let total_paper = gifts.iter().fold(0, |acc, gift| acc + gift.surface_area());
    let total_ribbon = gifts.iter().fold(0, |acc, gift| acc + gift.ribbon());

    println!("Day 2.");
    println!("  Total wrapping paper: {}", total_paper);
    println!("  Total ribbon: {}", total_ribbon);
}

