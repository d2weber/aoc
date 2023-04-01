pub fn groups() -> impl Iterator<Item = i32> {
    include_str!("input").split("\n\n").map(|group| {
        group
            .lines()
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .sum()
    })
}

#[test]
fn part1() {
    assert_eq!(groups().max().unwrap(), 67622);
}

#[test]
fn part2() {
    let mut top_3 = [i32::MIN; 3];

    groups().for_each(|v| {
        let mut curr: i32 = v;
        for r in top_3.iter_mut() {
            if curr > *r {
                std::mem::swap(&mut curr, &mut *r);
            }
        }
    });
    let result = top_3
        .iter()
        .inspect(|v| assert_ne!(**v, i32::MIN))
        .sum::<i32>();

    assert_eq!(result, 201491);
}
