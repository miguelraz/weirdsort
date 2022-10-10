#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Pair<'a> {
    s: &'a str,
    x: i32,
}

impl<'a> Pair<'a> {
    pub fn new(z: &'a str) -> Self {
        let p = z.split_once('-').unwrap();
        Pair {
            s: p.0,
            x: p.1.parse::<i32>().unwrap(),
        }
    }
}

fn main() {
    let mut v: Vec<Pair<'_>> = ["aa-2", "ab-100", "aa-10", "ba-25", "ab-3"]
        .into_iter()
        .map(Pair::new)
        .collect();
    v.sort_unstable();
    //println!("{v:?}");
}

#[test]
fn plz() {
    use crate::*;
    let mut v = ["aa-2", "ab-100", "aa-10", "ba-25", "ab-3"];
    let res = ["aa-2", "aa-10", "ab-3", "ab-100", "ba-25"];
    v.sort_unstable_by(|a, b| {
        let x = Pair::new(a);
        let y = &Pair::new(b);
        x.cmp(&y)
    });
    assert_eq!(v, res);
}
