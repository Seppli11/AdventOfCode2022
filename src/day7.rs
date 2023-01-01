use std::{
    cell::{Ref, RefCell},
    collections::{LinkedList, VecDeque},
    fs::{self, DirBuilder, DirEntry, File},
    future::Future,
    pin::Pin,
    rc::{Rc, Weak},
    task::{Context, Poll},
    time::Duration,
    vec,
};

use itertools::Itertools;

type RcCell<T> = Rc<RefCell<T>>;

const TOTAL_FS_SPACE: u32 = 70000000;
const UPDATE_SPACE: u32 = 30000000;

pub fn day7() {
    let input = fs::read_to_string("./input/day7/test-input.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day7/input.txt").expect("Couldn't load input");
    let mut parser = FSParser::new(input);
    parser.parse();
    let total_used_space = parser.root.borrow().get_size();
    let needed_space = UPDATE_SPACE - (TOTAL_FS_SPACE - total_used_space);
    let sum = parser
        .root
        .borrow()
        .directories()
        .map(|dir| dir.borrow().get_size())
        .filter(|size| *size >= needed_space)
        .min();
    println!("sum: {:?}", sum);
}

#[derive(Debug, Clone)]
struct DirectoryStruct {
    name: String,
    entries: Vec<RcCell<FileEntry>>,
    parent: Weak<RefCell<FileEntry>>,
}

impl DirectoryStruct {
    fn find_child(&self, name: &str) -> Option<RcCell<FileEntry>> {
        self.entries
            .iter()
            .find(|child| child.borrow().get_name() == name)
            .map(|rc| rc.clone())
    }

    fn get_size(&self) -> u32 {
        self.entries
            .iter()
            .map(|entry| entry.borrow().get_size())
            .sum()
    }
}

#[derive(Debug, Clone)]
enum FileEntry {
    File {
        name: String,
        size: u32,
        parent: Weak<RefCell<FileEntry>>,
    },
    Directory(DirectoryStruct),
}

impl FileEntry {
    fn new_folder(name: &str) -> FileEntry {
        FileEntry::new_child_folder(name, Weak::new())
    }
    fn new_child_folder(name: &str, parent: Weak<RefCell<FileEntry>>) -> FileEntry {
        FileEntry::Directory(DirectoryStruct {
            name: name.to_string(),
            entries: Vec::new(),
            parent,
        })
    }

    fn get_name(&self) -> &str {
        match self {
            FileEntry::File { name, .. } => name,
            FileEntry::Directory(dir) => &dir.name,
        }
    }

    fn get_parent(&self) -> Option<RcCell<FileEntry>> {
        match self {
            FileEntry::File { parent, .. } => parent.upgrade(),
            FileEntry::Directory(dir) => dir.parent.upgrade(),
        }
    }

    fn get_size(&self) -> u32 {
        match self {
            FileEntry::File { size, .. } => *size,
            FileEntry::Directory(dir) => dir.get_size(),
        }
    }

    fn as_directory_struct(&self) -> Option<&DirectoryStruct> {
        if let FileEntry::Directory(dir_struct) = self {
            Some(dir_struct)
        } else {
            None
        }
    }

    fn as_directory_struct_mut(&mut self) -> Option<&mut DirectoryStruct> {
        if let FileEntry::Directory(dir_struct) = self {
            Some(dir_struct)
        } else {
            None
        }
    }

    fn file_entries(&self) -> FileEntries {
        FileEntries::new(Rc::new(RefCell::new(self.clone())))
    }

    fn directories(&self) -> Directories<FileEntries> {
        Directories {
            iter: self.file_entries(),
        }
    }
}

struct FileEntries {
    stack: Vec<RcCell<FileEntry>>,
}

impl FileEntries {
    fn new(root: RcCell<FileEntry>) -> Self {
        Self {
            stack: vec![root.clone()],
        }
    }
}

impl Iterator for FileEntries {
    type Item = RcCell<FileEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.stack.pop();
        if let Some(next) = next {
            if let Some(dir_struct) = next.borrow().as_directory_struct() {
                let next_entries = dir_struct.entries.iter().map(|rc| rc.clone());
                self.stack.extend(next_entries);
            }
            Some(next)
        } else {
            None
        }
    }
}

struct Directories<T> {
    iter: T,
}

impl<T> Directories<T> where T: Iterator<Item = RcCell<FileEntry>> {}

impl<T> Iterator for Directories<T>
where
    T: Iterator<Item = RcCell<FileEntry>>,
{
    type Item = RcCell<FileEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.iter.next() {
            if matches!(*next.borrow(), FileEntry::Directory(..)) {
                return Some(next);
            }
        }
        None
    }
}

struct FSParser {
    input: VecDeque<String>,
    cwd: RcCell<FileEntry>,
    root: RcCell<FileEntry>,
}

impl FSParser {
    fn new(input: String) -> FSParser {
        let root_file_entry = FileEntry::new_folder("/");
        let root_rc_cell = RcCell::new(RefCell::new(root_file_entry));
        FSParser {
            input: input.lines().map(|str| str.into()).collect(),
            cwd: root_rc_cell.clone(),
            root: root_rc_cell,
        }
    }

    fn peek(&self) -> Option<&String> {
        self.input.front()
    }

    fn get(&mut self) -> Option<String> {
        self.input.pop_front()
    }

    fn parse(&mut self) {
        let root = self.cwd.clone();
        while let Some(line) = self.get() {
            let line = FSParser::parse_cmd(&line);
            if let Some(cmd) = line.first() {
                if *cmd == "cd" {
                    self.parse_cd(line[1])
                } else {
                    self.parse_ls()
                }
            }
        }
    }

    fn parse_cd(&mut self, folder: &str) {
        if folder == ".." {
            if let Some(parent) = self.cwd.clone().borrow().get_parent() {
                self.cwd = parent;
                println!("go up to {}", self.cwd.borrow().get_name())
            }
        } else if let Some(dir) = self.cwd.clone().borrow().as_directory_struct() {
            if let Some(parent) = dir.find_child(folder) {
                self.cwd = parent;
                println!("go into {}", self.cwd.borrow().get_name())
            }
        }
    }

    fn parse_ls(&mut self) {
        while let Some(line) = self.peek() {
            if line.starts_with("$") {
                break;
            }
            let parent = Rc::<RefCell<FileEntry>>::downgrade(&self.cwd);
            let splitted: Vec<&str> = line.split(" ").collect();
            let child = if splitted[0] == "dir" {
                FileEntry::new_child_folder(splitted[1], parent)
            } else {
                FileEntry::File {
                    name: splitted[1].to_string(),
                    size: splitted[0].parse().unwrap(),
                    parent,
                }
            };
            println!("found {:?}", child);
            self.cwd
                .borrow_mut()
                .as_directory_struct_mut()
                .map(move |dir_struct| dir_struct.entries.push(Rc::new(RefCell::new(child))));

            self.get();
        }
    }

    fn parse_cmd(line: &str) -> Vec<&str> {
        line.split(" ").filter(|str| *str != "$").collect()
    }
}
