use  {
    std::cell::RefCell,
    std::rc::Rc,
    std::option::Option,
    std::fmt::{
        Formatter,
        Display,
        Result
    }
};

const STACK_MAX_SIZE: usize = 526;
const IGCT: i32 = 8;

type SingleLink = Option<Rc<RefCell<Object>>>;

type StackLink = Rc<RefCell<Vec<SingleLink>>>;

#[derive(PartialEq, Clone)]
enum ObjectType {
    INT,
    TWIN
}

impl Display for ObjectType {

    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self)
    }
}

// impl std::cmp::PartialEq for ObjectType {
//     fn eq(&self, other: &ObjectType) -> bool {
//         self.INT == other.INT
//     }
// }

// fn parse<T>(obj: Option<Rc<RefCell<T>>>) -> Option<T> {
//     match obj {
//         Some(o) => {
//             Some(o.borrow().clone())
//         }
//         _ => None
//     }
// }

#[derive(Clone)]
struct Object {
    type_obj: ObjectType,
    marked: u8,
    next: SingleLink,
    value: i32,
    head: SingleLink,
    tail: SingleLink,
}

impl Object {

    pub fn new(type_obj: ObjectType, marked: u8, next: SingleLink, value: i32, head: SingleLink, tail: SingleLink) -> Self {
        Object {
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

    pub fn push(&mut self, item: Object) {
        self.stack_size += 1;
        self.stack.push(
            Some(Rc::new(
                RefCell::new(item)
            )
        ))
    }

    pub fn mark<'a>(&mut self, item: &'a mut Object) {
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

    // pub fn mark_all(&mut self) {
    //     for i in 0..self.stack_size {
    //         // self.mark(self.stack[i]);
    //         if let Some(ref stack_item) = self.stack[i] {
    //             self.mark(&mut stack_item.borrow_mut());
    //         }
    //     }
    // }

    pub fn markspeep(&mut self) {
        
    }

    pub fn gc(&mut self) {
        let past_num_objects: i32 = self.num_objects;

        // self.mark_all();
        self.markspeep();

        self.max_objects = self.num_objects * 2;

        println!("Collected {} objects, {} left", past_num_objects - self.num_objects, self.num_objects );
    }

    pub fn new_object(&mut self, type_obj: ObjectType) -> Object {
        if self.num_objects == self.max_objects { self.gc(); }

        let object: Object = Object::new(type_obj, 0, self.first_object.clone(), 0, None, None);

        self.num_objects += 1;

        object
    }

    pub fn push_int(&mut self, int_value: i32) {
        let mut object = self.new_object(ObjectType::INT);

        object.value = int_value;

        self.push(object);
    }

    pub fn push_pair(&mut self) {
        let object = self.new_object(ObjectType::TWIN);

        // object.tail = self.pop();
        // object.head = self.pop();

        self.push(object);
        // object.
    }

    // pub fn object_print(&self, object: Option<Object>) {
    //     if let Some(o) = object {
    //         match o.type_obj {
    //             ObjectType::INT => println!("{}", o.type_obj),
    //             ObjectType::TWIN => {
    //                 print!("(");
    //                 self.object_print(parse(o.head));
    //                 print!(", ");
    //                 self.object_print(parse(o.tail));
    //                 print!(")");
    //             }
    //         }
    //     }
    // }

    pub fn free(mut self) {
        self.stack_size = 0;
        self.gc();
        std::mem::forget(self);
    }
}

fn first_test() {
    println!("1: Objects on the stack are preserved.");
	let mut vm: VM = VM::init();
	vm.push_int(1);
	vm.push_int(2);
	vm.gc();
	vm.free();
}

fn second_test() {
    println!("2: Unreached objects are collected.");
	let mut vm: VM = VM::init();
	vm.push_int(1);
	vm.push_int(2);
    // vm.pop();
    // vm.pop();
	vm.gc();
	vm.free();
}

fn third_test() {
	println!("3: Reach the nested objects.");
	let mut vm: VM = VM::init();
	vm.push_int(1);
	vm.push_int(2);
	vm.push_pair();
	vm.push_int(3);
	vm.push_int(4);
	vm.push_int(5);
	vm.push_int(6);

	vm.gc();
	vm.free();
}

fn perfomance() {
    println!("Performance of GC.");
    let mut vm: VM = VM::init();
    for i in 0..1000 {
        for j in 0..20 {
            vm.push_int(i);
        }
        for k in 0..20 {
            // vm.pop();
        }
    }
    vm.free();
}

fn main() {
    first_test();
    second_test();
    third_test();
    perfomance();
}