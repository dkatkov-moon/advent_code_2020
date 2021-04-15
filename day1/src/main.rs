fn main() -> anyhow::Result<()>{
    let mut input_iter = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .map(Result::unwrap);
    dbg!(input_iter.next());

    Ok(())
}

fn find_pair_2020(numbers : Vec<i64>) -> Option<(i64, i64)> {
    todo!()
}
