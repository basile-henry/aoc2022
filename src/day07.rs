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

    let part1 =
        fs.0.reduce_dir_sizes(&|a, b| {
            let a = if a > 100000 { 0 } else { a };
            let b = if b > 100000 { 0 } else { b };
            a + b
        })
        .unwrap();

    let total_disk_space = 70000000;
    let need_unused = 30000000;
    let fs_size = fs.size();

    let dir_big_enough = |size| total_disk_space - fs_size + size >= need_unused;

    let part2 =
        fs.0.reduce_dir_sizes(&|a, b| {
            if dir_big_enough(a) && dir_big_enough(b) {
                a.min(b)
            } else if dir_big_enough(a) {
                a
            } else if dir_big_enough(b) {
                b
            } else {
                total_disk_space // bigger than we're hoping to find
            }
        })
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
            Cli::Output(Output::File(size, file)) => {
                self.0.insert_file(current, file, size);
            }
        }
    }

    fn size(&self) -> u64 {
        match self.0 {
            Node::Dir(size, _) => size,
            Node::File(size) => size,
        }
    }
}

#[derive(Debug)]
enum Node<'a, A: Allocator + Copy> {
    Dir(u64, HashMap<&'a str, Node<'a, A>, A>),
    File(u64),
}

impl<'a, A: Allocator + Copy> Node<'a, A> {
    fn empty_dir(alloc: A) -> Node<'a, A> {
        Node::Dir(0, hash_map!(alloc))
    }

    fn insert_dir(&mut self, path: &[&'a str], dir: &'a str) {
        match path {
            [] => match self {
                Node::Dir(_, hm) => {
                    _ = hm.insert(dir, Self::empty_dir(*hm.allocator()));
                }
                Node::File(_) => panic!("unexpected insert into file"),
            },
            [next, rest @ ..] => match self {
                Node::File(_) => {}
                Node::Dir(_, ref mut hm) => hm.get_mut(next).unwrap().insert_dir(rest, dir),
            },
        }
    }

    fn insert_file(&mut self, path: &[&'a str], file: &'a str, size: u64) {
        match path {
            [] => match self {
                Node::Dir(dir_size, hm) => {
                    *dir_size += size;
                    _ = hm.insert(file, Node::File(size));
                }
                Node::File(_) => panic!("unexpected insert into file"),
            },
            [next, rest @ ..] => match self {
                Node::File(_) => {}
                Node::Dir(dir_size, ref mut hm) => {
                    *dir_size += size;
                    hm.get_mut(next).unwrap().insert_file(rest, file, size);
                }
            },
        }
    }

    fn reduce_dir_sizes<F>(&self, f: &F) -> Option<u64>
    where
        F: Fn(u64, u64) -> u64,
    {
        match self {
            Node::Dir(dir_size, hm) => core::iter::once(*dir_size)
                .chain(hm.values().filter_map(|v| v.reduce_dir_sizes(f)))
                .reduce(f),
            Node::File(_) => None,
        }
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
