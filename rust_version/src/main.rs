struct Square(i8);

struct Bitboard {
    v: [u64; 2],
}

impl Iterator for Bitboard {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_lsb()
    }
}

impl Bitboard {
    const ALL: Bitboard = Bitboard {
        v: [0x7fff_ffff_ffff_ffff, 0x7fff_ffff_ffff_ffff],
    };
    fn value(&self, i: usize) -> u64 {
        self.v[i]
    }
    fn merge(&self) -> u64 {
        self.value(0) | self.value(1)
    }
    fn to_bool(&self) -> bool {
        self.merge() != 0
    }
    fn pop_lsb_right_unchecked(&mut self) -> Square {
        let sq = Square(self.value(0).trailing_zeros() as i8);
        self.v[0] &= self.v[0] - 1;
        sq
    }
    fn pop_lsb_left_unchecked(&mut self) -> Square {
        let sq = Square((self.value(1).trailing_zeros() + 64) as i8);
        self.v[1] &= self.v[1] - 1;
        sq
    }
    fn pop_lsb_unchecked(&mut self) -> Square {
        if self.value(0) != 0 {
            return self.pop_lsb_right_unchecked();
        }
        self.pop_lsb_left_unchecked()
    }
    fn pop_lsb(&mut self) -> Option<Square> {
        if self.to_bool() {
            Some(self.pop_lsb_unchecked())
        } else {
            None
        }
    }
}

fn main() {
    const NUM_TRIALS: i64 = 30000000;
    let mut sum: u64 = 0;
    let start = std::time::Instant::now();
    for _ in 0..NUM_TRIALS {
        let mut all_one: u64 = 0xffff_ffff_ffff_ffff;
        while all_one != 0 {
            sum += all_one.trailing_zeros() as u64;
            all_one &= all_one - 1;
        }
    }
    let end = start.elapsed();
    let elapsed = (end.as_secs() * 1000) as i64 + end.subsec_millis() as i64;
    println!("u64 bench");
    println!("elapsed: {} [msec]", elapsed);
    println!("times/s: {} [times/sec]", NUM_TRIALS * 1000 / elapsed);
    println!("sum: {}", sum);

    let mut sum: u64 = 0;
    let start = std::time::Instant::now();
    for _ in 0..NUM_TRIALS {
        let all_one = Bitboard::ALL;
        for sq in all_one {
            sum += sq.0 as u64;
        }
        //let mut all_one = Bitboard::ALL;
        //while all_one.to_bool() {
        //    let sq = all_one.pop_lsb_unchecked();
        //    sum += sq.0 as u64;
        //}
    }
    let end = start.elapsed();
    let elapsed = (end.as_secs() * 1000) as i64 + end.subsec_millis() as i64;
    println!("bitboard bench");
    println!("elapsed: {} [msec]", elapsed);
    if elapsed != 0 {
        println!("times/s: {} [times/sec]", NUM_TRIALS * 1000 / elapsed);
        println!("sum: {}", sum);
    }
}
