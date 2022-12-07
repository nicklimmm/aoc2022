use crate::utils::read_input_lines;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

type RefDirectory = Rc<RefCell<Directory>>;
type WeakRefDirectory = Weak<RefCell<Directory>>;

#[derive(Debug)]
struct Directory {
    children: HashMap<String, RefDirectory>,
    parent: Option<WeakRefDirectory>,
    size: u32,
}

impl Directory {
    fn new(parent: Option<WeakRefDirectory>) -> Self {
        Directory {
            children: HashMap::new(),
            parent,
            size: 0,
        }
    }

    fn from_input(file_name: &str) -> RefDirectory {
        let root = Self::build_init(file_name);
        Self::build_propagate_size(&root);
        root
    }

    fn build_init(file_name: &str) -> RefDirectory {
        let root = Rc::new(RefCell::new(Directory::new(None)));
        let mut input_lines_iter = read_input_lines(file_name);

        input_lines_iter.next();
        input_lines_iter.next();

        let mut cur = root.clone();
        for line in input_lines_iter {
            let args: Vec<&str> = line.split_whitespace().collect();

            match args[0] {
                "dir" => {
                    cur.borrow_mut().children.insert(
                        String::from(args[1]),
                        Rc::new(RefCell::new(Directory::new(Some(Rc::downgrade(&cur))))),
                    );
                }
                "$" => {
                    if args[1] == "cd" {
                        if args[2] == ".." {
                            let parent_ref = Weak::clone(cur.borrow().parent.as_ref().unwrap())
                                .upgrade()
                                .unwrap();
                            cur = parent_ref;
                        } else {
                            let child_ref = Rc::clone(
                                cur.borrow().children.get(&String::from(args[2])).unwrap(),
                            );
                            cur = child_ref;
                        }
                    }
                }
                _ => {
                    cur.borrow_mut().size += args[0].parse::<u32>().unwrap();
                }
            }
        }

        root
    }

    fn build_propagate_size(root: &RefDirectory) -> &RefDirectory {
        let mut children_size = 0;

        for subdir in root.borrow().children.values() {
            Self::build_propagate_size(subdir);
            children_size += subdir.borrow().size;
        }

        root.borrow_mut().size += children_size;
        root
    }
}

fn traverse_a(root: &RefDirectory, ans: &mut u32) {
    if root.borrow().size <= 100000 {
        *ans += root.borrow().size;
    }

    root.borrow().children.values().for_each(|d| {
        traverse_a(d, ans);
    });
}

pub fn solve_a() {
    let mut ans = 0;
    let root = Directory::from_input("input7.txt");
    traverse_a(&root, &mut ans);
    println!("{ans}");
}

fn traverse_b(root: &RefDirectory, ans: &mut u32, min_del_space: &u32) {
    for subdir in root.borrow().children.values() {
        traverse_b(subdir, ans, min_del_space);
    }

    let cur_size = root.borrow().size;
    if *min_del_space <= cur_size && cur_size < *ans {
        *ans = cur_size;
    }
}

pub fn solve_b() {
    let root = Directory::from_input("input7.txt");
    let min_del_space = 30_000_000 - (70_000_000 - root.borrow().size);
    let mut ans = u32::MAX;
    traverse_b(&root, &mut ans, &min_del_space);
    println!("{ans}");
}
