use std::collections::HashMap;
use std::path::{Path, PathBuf};

type FileSizes = HashMap<PathBuf, usize>;

/// Sizes of the directories, ignoring subdirectories
fn parse_sizes_direct_content(s: &str) -> FileSizes {
    let mut sizes = HashMap::<PathBuf, usize>::new();
    let mut cwd = PathBuf::new();
    s.lines().for_each(|line| {
        if line.starts_with("$ cd /") {
            cwd = PathBuf::from("/");
        } else if line.starts_with("$ cd ..") {
            cwd = cwd.parent().unwrap().to_owned();
        } else if let Some(dir) = line.strip_prefix("$ cd ") {
            cwd = cwd.join(dir);
        } else if line.starts_with("$ ls") {
            // return;
        } else if let Some(dir) = line.strip_prefix("dir ") {
            sizes.entry(cwd.join(dir)).or_insert(0);
        } else if let Some((new_size, _fname)) = line.split_once(' ') {
            let new_size: usize = new_size.parse().unwrap();
            if let Some(size) = sizes.get_mut(&cwd) {
                *size += new_size
            } else {
                sizes.insert(cwd.clone(), new_size);
            }
        } else {
            panic!("Cannot parse {line}");
        }
    });
    sizes
}

fn also_count_subdirectories(sizes: &mut FileSizes) {
    let mut keys: Vec<PathBuf> = sizes.keys().cloned().collect();
    keys.sort_unstable();
    keys.iter()
        .rev() // Start from leave nodes
        .filter(|dir| *dir != Path::new("/"))
        .for_each(|dir| {
            let size_to_add = *sizes.get(&*dir).unwrap();
            *sizes.get_mut(dir.parent().unwrap()).unwrap() += size_to_add;
        });
}

const SAMPLE: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

mod part1 {
    use super::*;
    fn solution(s: &str) -> usize {
        let mut sizes = parse_sizes_direct_content(s);
        also_count_subdirectories(&mut sizes);
        sizes.into_values().filter(|s| *s <= 100_000).sum()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 95437);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(include_str!("input")), 1642503);
    }
}

mod part2 {
    use super::*;
    fn solution(s: &str) -> usize {
        let mut sizes = parse_sizes_direct_content(s);
        let sizes_total: usize = sizes.values().sum();
        let required_free_up = sizes_total - 40_000_000;
        also_count_subdirectories(&mut sizes);
        sizes
            .into_values()
            .filter(|s| *s >= required_free_up)
            .min()
            .unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 24933642);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(include_str!("input")), 6999588);
    }
}
