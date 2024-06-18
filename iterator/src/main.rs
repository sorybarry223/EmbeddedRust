struct Counter { value: u8 }

impl Iterator for Counter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            255 => None,
            _   => { self.value += 1; Some(self.value) }
        }
    }
}

fn main() {
    let c = Counter{value: 0};
    // let mut i = c.map(|x| 2 * (x as u32));
    // while let Some(v) = i.next() {
    //     println!("v = {v:?}");
    // }
    let _ = c.map(|x| println!("{x:?}"));
}
