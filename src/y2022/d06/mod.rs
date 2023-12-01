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

pub const INPUT: &str = include_str!("input");

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        offset::<4>(s)
    }

    #[test]
    fn sample() {
        assert_eq!(solution("vwbjplbgvbhsrlpgdmjqwftvncz"), 4);
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 1655);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        offset::<14>(s)
    }

    #[test]
    fn sample() {
        assert_eq!(solution("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 2665);
    }
}
