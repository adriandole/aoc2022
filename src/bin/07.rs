use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
struct Dir {
    name: String,
    index: usize,
    parent: Option<usize>,
    children: HashMap<String, usize>,
    files: HashMap<String, usize>,
}
struct ArenaTree {
    arena: Vec<Dir>,
}

impl ArenaTree {
    fn new(&mut self, name: String, parent: Option<usize>) -> usize {
        let d = Dir {
            name: name,
            index: self.arena.len(),
            parent: parent,
            children: HashMap::new(),
            files: HashMap::new(),
        };
        self.arena.push(d);
        self.arena.len() - 1
    }

    fn get(&self, i: usize) -> &Dir {
        self.arena.get(i).unwrap()
    }

    fn get_mut(&mut self, i: usize) -> &mut Dir {
        self.arena.get_mut(i).unwrap()
    }

    fn size(&self, i: usize) -> usize {
        let mut total = 0;
        for (_, v) in &self.get(i).files {
            total += v;
        }
        for (_, v) in &self.get(i).children {
            total += self.size(*v);
        }
        total
    }
}

fn cd(fs: &mut ArenaTree, cwd: usize, target: &str) -> usize {
    match target {
        "/" => 0,
        ".." => fs.get(cwd).parent.unwrap(),
        s => {
            if !fs.get(cwd).children.contains_key(s) {
                let child = fs.new(s.into(), Some(cwd));
                fs.get_mut(cwd).children.insert(s.into(), child);
            }
            *fs.get(cwd).children.get(target).unwrap()
        }
    }
}

fn walk_dirs(input: &str) -> ArenaTree {
    let mut fs = ArenaTree { arena: vec![] };
    let sysroot = fs.new("/".into(), None);
    let mut cwd = sysroot;

    let mut it = input.split("\n").peekable();
    while let Some(cmd) = it.next() {
        if cmd.starts_with("$ cd") {
            let target = cmd.split_whitespace().last().unwrap();
            cwd = cd(&mut fs, cwd, target);
        } else if cmd.starts_with("$ ls") {
            while let Some(f) = it.peek() {
                if f.starts_with("$") {
                    break;
                }

                if f.starts_with("dir") {
                } else {
                    let line: Vec<_> = f.split_whitespace().collect();
                    let size = line[0].parse::<usize>().expect(f);
                    let name = line[1];
                    fs.get_mut(cwd).files.insert(name.into(), size);
                }
                it.next();
            }
        } else {
            panic!("Unknown line: {}", cmd);
        }
    }
    fs
}

pub fn part_one(input: &str) -> Option<usize> {
    let fs = walk_dirs(input);
    let mut total = 0;
    for i in 0..fs.arena.len() {
        let dsize = fs.size(i);
        if dsize <= 100000 {
            total += dsize;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let fs = walk_dirs(input);
    let disk_size = 70000000;
    let needed_space = 30000000;
    let used_space = fs.size(0);
    let free_space = disk_size - used_space;
    let must_free = needed_space - free_space;

    let mut sizes: Vec<usize> = Vec::new();
    for i in 0..fs.arena.len() {
        sizes.push(fs.size(i));
    }
    sizes.sort();
    let k = sizes.iter().find(|s| **s >= must_free);
    Some(*k.unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
