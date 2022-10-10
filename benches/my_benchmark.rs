use criterion::{black_box, criterion_group, criterion_main, Criterion};
//use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::*;
//println!("Random string: {}", string);

#[derive(Debug)]
// create a struct with a String and an integer
// not using &str due to lifetime issues
struct Pair0 {
    x: String,
    y: i32,
}

fn author(v: &[String]) {
    // input data vector
    //let v = vec!["aa-2", "ab-100", "aa-10", "ba-25", "ab-3"];
    // create an accumulating vector of `Pair`s
    let mut res: Vec<Pair0> = vec![];
    // for each string, split at '-',
    //  convert the first part to String and the second to integer.
    //  then push onto the accumulator
    for s in v {
        let a: Vec<&str> = s.split("-").collect();
        let tmp_pair = Pair0 {
            x: a[0].to_string(),
            y: a[1].parse::<i32>().unwrap(),
        };
        res.push(tmp_pair);
    }
    // sort by Pair.x then Pair.y
    res.sort_by_key(|k| (k.x.clone(), k.y.clone()));
    // start building a new vector for the final result
    let mut res2: Vec<String> = vec![];
    // paste together Pair.x, '-', and Pair.y (as String)
    for s2 in res {
        res2.push(s2.x + "-" + &s2.y.to_string());
    }

    // ["aa-2", "aa-10", "ab-3", "ab-100", "ba-25"]
    //println!("{:?}", res2);
}

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

fn miguel_unstable(v: &[String]) {
    //let mut v: Vec<Pair<'_>> = vec!["aa-2", "ab-100", "aa-10", "ba-25", "ab-3"]
    let mut v: Vec<Pair<'_>> = v.into_iter().map(|s| Pair::new(s)).collect();
    v.sort_unstable();
    //println!("{v:?}");
}
fn miguel_stable(v: &[String]) {
    let mut v: Vec<Pair<'_>> = v.into_iter().map(|s| Pair::new(s)).collect();
    v.sort_by(|a, b| a.cmp(&b));
}
pub fn data(n: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let abc: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(format!(
            "{}{}-{}",
            abc.choose(&mut rng).unwrap(),
            abc.choose(&mut rng).unwrap(),
            rand::random::<u8>()
        ));
    }
    v
}
// const p1: Pair<'_> = Pair("aa-2");
// const array: [Pair<'_>; 5] = [
//     Pair("aa-2"),
//     Pair("ab-100"),
//     Pair("aa-10"),
//     Pair("ba-25"),
//     Pair("ab-3"),
// ];

pub fn weirdsort_benchmark(c: &mut Criterion) {
    //let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    //let v = vec!["aa-2", "ab-100", "aa-10", "ba-25", "ab-3"];
    let v = data(10_000);
    c.bench_function("author 10_000", |b| b.iter(|| author(black_box(&v[..]))));
    c.bench_function("miguel_stable 10_00", |b| {
        b.iter(|| miguel_stable(black_box(&v)))
    });
    c.bench_function("miguel_unstable 10_000", |b| {
        b.iter(|| miguel_unstable(black_box(&v)))
    });
    let z = v.clone();
    let mut z: Vec<Pair<'_>> = z.iter().map(|s| Pair::new(s.as_str())).collect();
    c.bench_function("stable 10_000", |b| b.iter(|| z.sort()));
    let z = v.clone();
    let mut z: Vec<Pair<'_>> = z.iter().map(|s| Pair::new(s.as_str())).collect();
    c.bench_function("unstable 10_000", |b| b.iter(|| z.sort_unstable()));
}

criterion_group!(benches, weirdsort_benchmark);
criterion_main!(benches);
