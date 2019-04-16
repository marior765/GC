use std::cell::RefCell;
use std::rc::Rc;

#[warn(dead_code)]
const STACK_MAX_SIZE: usize = 526;
const IGCT: i32 = 8;

type SingleLink = Option<Rc<RefCell<s_object>>>;

type StackLink = Rc<RefCell<Vec<SingleLink>>>;

#[derive(PartialEq)]
enum ObjectType {
    INT,
    TWIN
}

// impl std::cmp::PartialEq for ObjectType {
//     fn eq(&self, other: &ObjectType) -> bool {
//         self.INT == other.INT
//     }
// }

struct s_object {
    type_obj: ObjectType,
    marked: u8,
    next: SingleLink,
    value: i32,
    head: SingleLink,
    tail: SingleLink,
}

struct VM {
    stack: Vec<SingleLink>,
    stack_size: usize,
    first_object: SingleLink,
    num_objects: i32,
    max_objects: i32,
}

impl VM {

    pub fn init() -> Self {
        VM {
            stack: Vec::new(),
            stack_size: 0,
            first_object: None,
            num_objects: 0,
            max_objects: IGCT,
        }
    }

    // pub fn pop(&mut self) -> SingleLink {
    //     self.stack[0]
    // }

    pub fn push(&mut self, item: s_object) {
        self.stack_size += 1;
        self.stack.push(
            Some(Rc::new(
                RefCell::new(item)
            )
        ))
    }

    pub fn mark<'a>(&mut self, item: &'a mut s_object) {
        if item.marked != 1 {
            item.marked = 1;
        }

        if item.type_obj == ObjectType::TWIN {
            match item.head {
                Some(ref h) => {
                    self.mark(&mut h.borrow_mut())
                }
                None => println!("Head not found!")
            }
            match item.tail {
                Some(ref t) => {
                    self.mark(&mut t.borrow_mut())
                }
                None => println!("Tail not found!")
            }
        }

    }

    pub fn mark_all(&mut self) {
        for i in 0..self.stack_size {
            // self.mark(self.stack[i]);
            if let Some(ref stack_item) = self.stack[i] {
                self.mark(&mut stack_item.borrow_mut());
            }
        }
    }
}