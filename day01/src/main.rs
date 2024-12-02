use std::cmp;
use std::collections::HashMap;
use std::io;

const PART_ONE: bool = false;

fn main() {
    println!("part one? {}", PART_ONE);
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    let stdin = io::stdin();
    let lines = stdin.lines();
    for line_result in lines {
        let line = line_result.unwrap();
        let (lval, rval) = parse(&line).unwrap();
        left.push(lval);
        right.push(rval);
    }

    println!(
        "got {} left values, {} right values",
        left.len(),
        right.len()
    );

    let sum: u32 = if PART_ONE {
        left.sort();
        right.sort();
        assert_eq!(left.len(), right.len());

        left.iter()
            .zip(right.iter())
            .fold(0_u32, |acc, (l, r)| acc + diff(*l, *r))
    } else {
        let mut right_occurrences: HashMap<u32, u8> = HashMap::new();
        for r in right {
            *right_occurrences.entry(r).or_insert(0) += 1;
        }
        left.into_iter().fold(0_u32, |acc, l| {
            let val = right_occurrences.get(&l);
            let occ: u32 = val.map(|v| *v).unwrap_or_default().into();
            acc + l * occ
        })
    };

    println!("got sum: {}", sum);
}

fn parse(line: &String) -> io::Result<(u32, u32)> {
    // println!("got line: `{}`", line);

    let split = line.trim().split_whitespace().collect::<Vec<_>>();

    if split.len() != 2 {
        return Result::Err(io::Error::other(format!(
            "expected only two chunks in `{}`, got {}",
            line,
            split.len()
        )));
    }

    return Result::Ok((
        split[0]
            .parse()
            .expect(format!("expected left {} to be a u16", split[0]).as_str()),
        split[1]
            .parse()
            .expect(format!("expected right {} to be a u16", split[1]).as_str()),
    ));
}

fn diff(l: u32, r: u32) -> u32 {
    return cmp::max(l, r) - cmp::min(l, r);
}
