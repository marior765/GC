use std::cell::RefCell;
use std::rc::Rc;

#[warn(dead_code)]
const STACK_MAX_SIZE: usize = 526;
const IGCT: i32 = 8;

type SingleLink = Option<Rc<RefCell<s_object>>>;

type StackLink = Rc<RefCell<Vec<SingleLink>>>;

#[derive(PartialEq, Clone)]
enum ObjectType {
    INT,
    TWIN
}

// impl std::cmp::PartialEq for ObjectType {
//     fn eq(&self, other: &ObjectType) -> bool {
//         self.INT == other.INT
//     }
// }

#[derive(Clone)]
struct s_object {
    type_obj: ObjectType,
    marked: u8,
    next: SingleLink,
    value: i32,
    head: SingleLink,
    tail: SingleLink,
}

impl s_object {

    pub fn new(type_obj: ObjectType, marked: u8, next: SingleLink, value: i32, head: SingleLink, tail: SingleLink) -> Self {
        s_object {
            type_obj,
            marked,
            next,
            value,
            head,
            tail
        }
    }
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

    pub fn markspeep(&mut self) {
        
    }

    pub fn gc(&mut self) {
        let past_num_objects: i32 = self.num_objects;

        self.mark_all();
        self.markspeep();

        self.max_objects = self.num_objects * 2;

        println!("Collected {} objects, {} left", past_num_objects - self.num_objects, self.num_objects );
    }

    pub fn new_object(&mut self, type_obj: ObjectType) -> s_object {
        if self.num_objects == self.max_objects { self.gc(); }

        let object: s_object = s_object::new(type_obj, 0, &mut self.first_object.cloned(), 0, None, None);

        self.num_objects += 1;

        object
    }

    pub fn push_int(self, int_value: i32) {
        let mut object = self.new_object(ObjectType::INT);

        object.value = int_value;

        self.push(object);
    }

    pub fn push_pair(self) -> s_object {
        let mut object = self.new_object(ObjectType::TWIN);

        // object.tail = self.pop();
        // object.head = self.pop();

        self.push(object);
        object
    }

    pub fn object_print(self, object: s_object) {
        match object.type_obj {
            ObjectType::INT => println!("{}", object.type_obj),
            ObjectType::TWIN => {
                print!("(");
                self.object_print(object.head);
                print!(", ");
                self.object_print(object.tail);
                print!(")");
            }
        }
    }

    pub fn free(mut self) {
        self.stack_size = 0;
        self.gc();
        std::mem::forget(self);
    }
}