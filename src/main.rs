#[warn(dead_code)]
const STACK_MAX_SIZE: usize = 526;
const IGCT: i32 = 8;

enum ObjectType {
    INT,
    TWIN
}

struct s_object {
    type_obj: ObjectType,
    marked: u8,
    next: Box<s_object>,
    // unsafe {
    //     let value: i32;
    //     struct {
    //         let head: *sObject;
    //         let tail: *sObject;
    //     }
    // }
}

struct VM {
    stack: [Option<s_object>; STACK_MAX_SIZE],
    stack_size: i32,
    first_object: Option<s_object>,
    num_objects: i32,
    max_objects: i32,
}

impl VM {

    pub fn init() -> Self {
        VM {
            stack: [None, STACK_MAX_SIZE],
            stack_size: 0,
            first_object: None,
            num_objects: 0,
            max_objects: IGCT,
        }
    }
}