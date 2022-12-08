pub fn solve(input: String) -> String {
    let input = input.trim();
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

const FILESYSTEM_SIZE: usize = 70_000_000;
const SPACE_NEEDED: usize = 30_000_000;

#[derive(Debug, PartialEq, Eq)]
enum Inode {
    Dir {
        name: String,
        size: usize,
        inodes: Vec<Inode>,
    },
    File {
        name: String,
        size: usize,
    },
}

#[derive(Debug)]
enum DirTarget {
    Root,
    Named { name: String },
    Up,
}

#[derive(Debug)]
enum Command {
    ChangeDir { to: DirTarget },
    List,
}

impl Inode {
    fn cd_mut<'a>(&'a mut self, to: &str) -> Option<&'a mut Self> {
        let Inode::Dir { inodes, .. } = self else {
            panic!("can't cd from a file");
        };
        inodes.iter_mut().find(|inode| {
            if let Self::Dir { name, .. } = inode {
                name == to
            } else {
                false
            }
        })
    }

    fn push_inode(&mut self, inode: Inode) {
        if let Self::Dir { inodes, .. } = self {
            inodes.push(inode);
        } else {
            panic!("can't add stuff to a file")
        }
    }

    fn compute_size(&mut self) -> usize {
        match self {
            Inode::Dir { inodes, size, .. } => {
                *size = inodes.iter_mut().map(Self::compute_size).sum();
                *size
            }
            Inode::File { size, .. } => *size,
        }
    }
}

impl core::str::FromStr for Inode {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, ()> {
        if let Some((_, dir_name)) = line.split_once("dir ") {
            return Ok(Self::Dir {
                name: dir_name.into(),
                size: 0,
                inodes: Vec::new(),
            });
        }
        let (size, name) = line.split_once(' ').ok_or(())?;
        Ok(Self::File {
            name: name.into(),
            size: size.parse().map_err(drop)?,
        })
    }
}

impl core::str::FromStr for DirTarget {
    type Err = ();
    fn from_str(target: &str) -> Result<Self, ()> {
        Ok(match target {
            "/" => Self::Root,
            ".." => DirTarget::Up,
            name => DirTarget::Named { name: name.into() },
        })
    }
}

impl core::str::FromStr for Command {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, ()> {
        let (_, cmd_str) = line.split_once("$ ").ok_or(())?;

        if let Some((_, target)) = cmd_str.split_once("cd ") {
            Ok(Self::ChangeDir {
                to: target.parse().unwrap(),
            })
        } else if cmd_str == "ls" {
            Ok(Command::List)
        } else {
            Err(())
        }
    }
}

fn rebuild_fs(input: &str) -> Inode {
    let mut lines = input.trim().lines().peekable();
    let mut fs: Inode = Inode::Dir {
        name: "/".into(),
        size: 0,
        inodes: Vec::new(),
    };

    // track pwd as a vec of directory names
    // empty vec == at the root
    let mut pwd: Vec<String> = Vec::new();

    'all: while let Some(cmd_line) = lines.next() {
        // expect a command
        match cmd_line.parse().unwrap() {
            Command::List => loop {
                let Some(next_line) = lines.peek() else {
                    break 'all;
                };
                if next_line.starts_with("$ ") {
                    break;
                }

                // read inode lines and add to pwd
                let inode: Inode = lines.next().unwrap().parse().unwrap();
                let mut dir = &mut fs;
                for dir_name in &pwd {
                    dir = dir.cd_mut(dir_name).unwrap();
                }
                dir.push_inode(inode);
            },
            Command::ChangeDir { to } => match to {
                DirTarget::Root => pwd.clear(),
                DirTarget::Named { name } => pwd.push(name),
                DirTarget::Up => drop(pwd.pop()),
            },
        }
    }

    // fill in directory sizes
    fs.compute_size();

    fs
}

// scan a dir, and return the sum of each dir size <= 100_000
fn dfs_part1(dir: &Inode) -> usize {
    let Inode::Dir { size, inodes, .. } = dir else {
        // files don't get counted
        return 0;
    };

    // scan each inode, collecting to a sum
    let mut sum = inodes.iter().map(dfs_part1).sum();
    if *size <= 100_000 {
        sum += size;
    }

    sum
}

fn part1(input: &str) -> usize {
    let fs = rebuild_fs(input);
    dfs_part1(&fs)
}

// scan a dir, and return the sum of each dir size <= 100_000
fn dfs_part2(dir: &Inode, size_goal: usize, best_fit: &mut usize) {
    let Inode::Dir { size, inodes, .. } = dir else {
        // files don't get counted
        return;
    };

    if *size >= size_goal {
        // scan children that are smaller and potentially better fits
        for ii in inodes {
            dfs_part2(ii, size_goal, best_fit);
        }

        // apply this size if it's the best
        *best_fit = (*best_fit).min(*size);
    }
}

fn part2(input: &str) -> usize {
    let fs = rebuild_fs(input);
    let Inode::Dir { size: space_taken, .. } = fs else {
        panic!("root is not a dir");
    };

    let free_disk_space = FILESYSTEM_SIZE - space_taken;
    if free_disk_space >= SPACE_NEEDED {
        return 0;
    }
    let delete_at_least = SPACE_NEEDED - free_disk_space;

    let mut best_fit = usize::MAX;
    dfs_part2(&fs, delete_at_least, &mut best_fit);
    best_fit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            95437,
            part1(
                "$ cd /
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
7214296 k"
            )
        );
    }
}
