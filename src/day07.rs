use alloc::vec::Vec;
use core::alloc::Allocator;
use core::fmt::Debug;

use crate::hash::HashMap;
use crate::hash_map;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn day07<A: Allocator + Copy + Debug>(alloc: A, input: &str) -> (u64, u64) {
    let fs = FileSystem::from_cli_input(alloc, input);

    let part1 = fs
        .0
        .filter_map_reduce_dir_sizes(&|x| if x > 100000 { None } else { Some(x) }, &|a, b| a + b)
        .unwrap();

    let total_disk_space = 70000000;
    let need_unused = 30000000;
    let fs_size = fs.size();

    let part2 =
        fs.0.filter_map_reduce_dir_sizes(
            &|x| {
                let dir_big_enough = total_disk_space - fs_size + x >= need_unused;
                if dir_big_enough {
                    Some(x)
                } else {
                    None
                }
            },
            &|a, b| a.min(b),
        )
        .unwrap();

    (part1, part2)
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
    Ls,
    CdUp,
    CdDown(&'a str),
}

impl<'a> Command<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Command<'a>, ()> {
        preceded(
            tag("$ "),
            alt((
                map(tag("ls"), |_| Command::Ls),
                map(tag("cd .."), |_| Command::CdUp),
                map(preceded(tag("cd "), not_line_ending), Command::CdDown),
            )),
        )(input)
    }
}

#[derive(Debug, PartialEq)]
enum Output<'a> {
    Dir(&'a str),
    File(u64, &'a str),
}

impl<'a> Output<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Output<'a>, ()> {
        alt((
            map(preceded(tag("dir "), not_line_ending), Output::Dir),
            map(
                separated_pair(u64, char(' '), not_line_ending),
                |(size, file)| Output::File(size, file),
            ),
        ))(input)
    }
}

#[derive(Debug, PartialEq)]
enum Cli<'a> {
    Command(Command<'a>),
    Output(Output<'a>),
}

impl<'a> Cli<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Cli<'a>, ()> {
        alt((
            map(Command::parse, Cli::Command),
            map(Output::parse, Cli::Output),
        ))(input)
    }
}

#[derive(Debug)]
struct FileSystem<'a, A: Allocator + Copy>(Node<'a, A>);

impl<'a, A: Allocator + Copy> FileSystem<'a, A> {
    fn from_cli_input(alloc: A, input: &'a str) -> FileSystem<'a, A> {
        fold_many1(
            terminated(Cli::parse, newline),
            || (FileSystem::new(alloc), Vec::new_in(alloc)),
            |(mut fs, mut current), cli| {
                fs.discover_step(&mut current, cli);
                (fs, current)
            },
        )(input)
        .unwrap()
        .1
         .0
    }

    fn new(alloc: A) -> FileSystem<'a, A> {
        FileSystem(Node::empty_dir(alloc))
    }

    fn discover_step(&mut self, current: &mut Vec<&'a str, A>, cli: Cli<'a>) {
        match cli {
            Cli::Command(Command::Ls) => {}
            Cli::Command(Command::CdUp) => {
                current.pop().unwrap();
            }
            Cli::Command(Command::CdDown("/")) => {}
            Cli::Command(Command::CdDown(dir)) => {
                current.push(dir);
            }
            Cli::Output(Output::Dir(dir)) => {
                self.0.insert_dir(current, dir);
            }
            Cli::Output(Output::File(size, _file)) => {
                self.0.insert_file(current, size);
            }
        }
    }

    fn size(&self) -> u64 {
        self.0.dir_size
    }
}

#[derive(Debug)]
struct Node<'a, A: Allocator + Copy> {
    dir_size: u64,
    dirs: HashMap<&'a str, Node<'a, A>, A>,
}

impl<'a, A: Allocator + Copy> Node<'a, A> {
    fn empty_dir(alloc: A) -> Node<'a, A> {
        Node {
            dir_size: 0,
            dirs: hash_map!(alloc),
        }
    }

    fn insert_dir(&mut self, path: &[&'a str], dir: &'a str) {
        match path {
            [] => {
                self.dirs
                    .insert(dir, Self::empty_dir(*self.dirs.allocator()));
            }
            [next, rest @ ..] => {
                self.dirs.get_mut(next).unwrap().insert_dir(rest, dir);
            }
        }
    }

    fn insert_file(&mut self, path: &[&'a str], size: u64) {
        match path {
            [] => {
                self.dir_size += size;
            }
            [next, rest @ ..] => {
                self.dir_size += size;
                self.dirs.get_mut(next).unwrap().insert_file(rest, size);
            }
        }
    }

    fn filter_map_reduce_dir_sizes<M, R>(&self, m: &M, r: &R) -> Option<u64>
    where
        M: Fn(u64) -> Option<u64>,
        R: Fn(u64, u64) -> u64,
    {
        m(self.dir_size)
            .into_iter()
            .chain(
                self.dirs
                    .values()
                    .filter_map(|v| v.filter_map_reduce_dir_sizes(m, r)),
            )
            .reduce(r)
    }
}

#[test]
fn parse() {
    assert_eq!(Command::parse("$ cd .."), Ok(("", Command::CdUp)));
    assert_eq!(Command::parse("$ cd /"), Ok(("", Command::CdDown("/"))));
    assert_eq!(Command::parse("$ ls"), Ok(("", Command::Ls)));
}

#[test]
fn both_parts() {
    let bump = bumpalo::Bump::new();
    let example = r#"$ cd /
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
7214296 k
"#;
    assert_eq!(day07(&bump, example).0, 95437);
    assert_eq!(day07(&bump, example).1, 24933642);
}
