use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Struct for representing the filesystem
#[derive(Debug, Clone)]
struct Directory {
    /// Vector of subdirectories
    subdirs: Vec<Directory>,
    /// Aggregated size of subfiles
    files: usize,
}

impl Directory {
    /// Creates a new empty directory
    fn new() -> Self {
        Self {
            subdirs: Vec::new(),
            files: 0,
        }
    }

    /// Returns the total size of the directory
    fn size(&self) -> usize {
        let mut res = self.files;
        for subdir in self.subdirs.iter() {
            res += subdir.size();
        }
        res
    }

    /// Adds a subfile to the directory
    fn add_file(&mut self, file_size: usize) {
        self.files += file_size;
    }

    /// Adds a subdirectory to the directory
    fn add_subdir(&mut self, subdir: Directory) {
        self.subdirs.push(subdir);
    }
}

/// Reads the input file and returns the entire filesystem.
fn read_input(path: &str) -> Directory {
    // We will follow a stack approach to complete the tree.
    let mut dir_stack: Vec<Directory> = Vec::new();

    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        let line = line.expect("error parsing line");
        let command: Vec<&str> = line.split_whitespace().collect();

        if command[0] == "$" {
            if command[1] == "cd" {
                if command[2] == ".." {
                    // The cd .. command tell us that the directory on top of the stack is done
                    // We pop it from the stack and add it as a subdirectory to the next directory.
                    let subdir = dir_stack.pop().expect("error popping directory");
                    let dir_len = dir_stack.len();
                    dir_stack[dir_len - 1].add_subdir(subdir);
                } else {
                    // The cd <dirname> tell us that we need to add a new directory to the stack
                    dir_stack.push(Directory::new());
                }
            } else if command[1] == "ls" {
                continue; // We can ignore this command
            } else {
                panic!("unknown command")
            }
        } else if command[0] == "dir" {
            continue; // We can ignore this command
        } else {
            // If the line represents a file, we add its size to the directory on top of the stack
            let file_size = command[0]
                .parse::<usize>()
                .expect("unable to parse file size");
            let dir_len = dir_stack.len();
            dir_stack[dir_len - 1].add_file(file_size);
        }
    }
    // After processing the file, we consume the stack until there is only the root directory
    while dir_stack.len() > 1 {
        let subdir = dir_stack.pop().expect("error popping directory");
        let dir_len = dir_stack.len();
        dir_stack[dir_len - 1].add_subdir(subdir);
    }
    // We return the root directory
    dir_stack.pop().expect("error popping directory")
}

/// We sum the size of all the directories whose size is less than or equal to `at_most`.
fn exercise_1(dir: &Directory, at_most: usize) -> usize {
    let mut res = 0;
    let root_size = dir.size();
    if root_size <= at_most {
        res += root_size;
    }
    for item in dir.subdirs.iter() {
        res += exercise_1(item, at_most);
    }
    res
}

/// We return the size of the smallest directory that we can remove while saving `space` bits.
fn exercise_2(dir: &Directory, space: usize) -> Option<usize> {
    let dir_size = dir.size();
    if dir_size < space {
        // This directory is too small and we cannot consider to remove it to save space
        return None;
    }
    // So far, the best option is to remove this directory.
    let mut min_size = dir_size;
    // Let's look iteratively through the subdirectories for a better solution:
    for subdir in dir.subdirs.iter() {
        if let Some(subdir_size) = exercise_2(subdir, space) {
            // If the subdirectory (or one of its subdirectories) is big enough
            // but smaller than the currently best solution, we update min_size
            if subdir_size < min_size {
                min_size = subdir_size;
            }
        }
    }
    Some(min_size)
}

fn main() {
    // First we read the input file.
    let x = read_input("data/7_input.txt");
    println!("{:?}", exercise_1(&x, 100000));
    let required_space = 30000000 - (70000000 - x.size());
    let min_space = exercise_2(&x, required_space);
    println!("{:?}", min_space);
    if let Some(space) = min_space {
        println!("Now we have {} of free space", 70000000 - x.size() + space);
    }
}
