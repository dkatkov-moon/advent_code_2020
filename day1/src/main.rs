use itertools::Itertools;

fn main() -> anyhow::Result<()>{
    let (a, b) = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
        .expect("no pair");

    dbg!(a+b);
    dbg!(a*b);

    Ok(())
}