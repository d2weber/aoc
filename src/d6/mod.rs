use std::collections::HashSet;
use std::collections::VecDeque;

fn interleaved_chunks<const N: usize, T>(
    it: impl Iterator<Item = T>,
) -> impl Iterator<Item = [T; N]>
where
    [T; N]: for<'a> TryFrom<&'a mut [T]>,
{
    let mut buffer = VecDeque::new();
    it.filter_map(move |v| {
        if buffer.len() > N - 1 {
            buffer.pop_front();
        }
        buffer.push_back(v);
        buffer.make_contiguous().try_into().ok()
    })
}

fn offset<const N: usize>(input: &str) -> usize {
    assert_eq!(input.len(), input.bytes().len());
    interleaved_chunks::<N, _>(input.bytes())
        .zip(N..)
        .find_map(|(a, i)| {
            let mut set = HashSet::new();
            if a.iter().all(move |v| set.insert(v)) {
                Some(i)
            } else {
                None
            }
        })
        .unwrap()
}

mod part1 {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(offset::<4>("vwbjplbgvbhsrlpgdmjqwftvncz"), 4);
        assert_eq!(offset::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(offset::<4>("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(offset::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(offset::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    #[test]
    fn actual() {
        assert_eq!(offset::<4>(include_str!("input")), 1655);
    }
}
mod part2 {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(offset::<14>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(offset::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(offset::<14>("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(offset::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(offset::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
    #[test]
    fn actual() {
        assert_eq!(offset::<14>(include_str!("input")), 2665);
    }
}
