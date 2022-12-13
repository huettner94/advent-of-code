use itertools::Itertools;

use crate::common::read_file_to_lines;

#[derive(Debug)]
enum Element {
    File {
        name: String,
        size: u64,
    },
    Directory {
        name: String,
        elements: Vec<Element>,
    },
}

impl Element {
    fn new_dir(name: String) -> Element {
        Element::Directory {
            name,
            elements: Vec::new(),
        }
    }

    fn add_dir(&mut self, name: String) {
        match self {
            Element::File { name: _, size: _ } => panic!(),
            Element::Directory { name: _, elements } => elements.push(Element::new_dir(name)),
        }
    }

    fn add_file(&mut self, name: String, size: u64) {
        match self {
            Element::File { name: _, size: _ } => panic!(),
            Element::Directory { name: _, elements } => elements.push(Element::File { name, size }),
        }
    }

    fn get_size(&self) -> u64 {
        match self {
            Element::File { name: _, size } => *size,
            Element::Directory { name: _, elements } => elements.iter().map(|e| e.get_size()).sum(),
        }
    }

    fn get_name(&self) -> &String {
        match self {
            Element::File { name, size: _ } => name,
            Element::Directory { name, elements: _ } => name,
        }
    }

    fn get_by_path(&mut self, mut path: Vec<String>) -> Option<&mut Element> {
        assert!(!path.is_empty());
        if path.len() == 1 && &path[0] == self.get_name() {
            return Some(self);
        }
        match self {
            Element::Directory { name, elements } => {
                if &path[0] == name {
                    path.remove(0);
                    for e in elements {
                        if &path[0] == e.get_name() {
                            return e.get_by_path(path);
                        }
                    }
                }
                None
            }
            Element::File { name: _, size: _ } => None,
        }
    }

    fn parse_from_string(lines: Vec<String>) -> Element {
        let mut path: Vec<String> = vec![String::new()];
        let mut root = Element::new_dir(String::new());
        let mut lineiter = lines.iter().peekable();
        while let Some(line) = lineiter.next() {
            assert!(line.starts_with('$'));
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[1] {
                "cd" => match parts[2] {
                    "/" => path = vec![String::new()],
                    ".." => {
                        path.pop();
                    }
                    v => path.push(v.to_string()),
                },
                "ls" => {
                    let curdir = root.get_by_path(path.clone()).unwrap();
                    while lineiter.peek().is_some() && !lineiter.peek().unwrap().starts_with('$') {
                        let line = lineiter.next().unwrap();
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        match parts[0] {
                            "dir" => curdir.add_dir(parts[1].to_string()),
                            v => {
                                let size = v.parse::<u64>().unwrap();
                                curdir.add_file(parts[1].to_string(), size);
                            }
                        }
                    }
                }
                _ => panic!(),
            }
        }
        root
    }
}

fn get_sizes(e: &Element) -> Vec<(String, u64)> {
    let mut out = Vec::new();
    match e {
        Element::File { name: _, size: _ } => {}
        Element::Directory { name, elements } => {
            let size = e.get_size();
            out.push((name.clone(), size));
            for e in elements {
                out.append(&mut get_sizes(e));
            }
        }
    }
    out
}

pub fn run_1() {
    let lines = read_file_to_lines("src/day_07.input");
    let root = Element::parse_from_string(lines);
    let sizes = get_sizes(&root);
    println!(
        "{}",
        sizes
            .iter()
            .map(|(_, e)| e)
            .filter(|e| e < &&100000)
            .sum::<u64>()
    );
}

pub fn run_2() {
    let lines = read_file_to_lines("src/day_07.input");
    let root = Element::parse_from_string(lines);
    let sizes = get_sizes(&root);
    let needed_free_size = 30000000 - (70000000 - root.get_size());
    let best = sizes
        .iter()
        .filter(|(_, s)| s >= &needed_free_size)
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .next()
        .unwrap();
    println!("{:?}", best);
}
