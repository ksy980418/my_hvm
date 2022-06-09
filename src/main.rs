use std::env;
use std::process;
use std::fs;

const MEM_SIZE: usize = 16384;
static mut MEM: [i32; MEM_SIZE] = [0; MEM_SIZE];
static mut _STACK: Vec<i32> = Vec::new();
static mut _CALL_STACK: Vec<i32> = Vec::new();

fn mem_read(idx: usize) -> i32 {
    unsafe { MEM[idx] }
}

fn mem_write(idx: usize, val: i32) {
    unsafe { MEM[idx] = val; }
}

fn stack_push(val: i32) {
    unsafe { _STACK.push(val); }
}

fn stack_pop() -> i32 {
    match unsafe { _STACK.pop() } {
        Some(n) => {n}
        None => {
            eprintln!("stack pop error");
            process::exit(0);
        }
    }   
}

fn stack_read(idx: usize) -> i32 {
    let length = unsafe { _STACK.len() };
    if length == 0 || idx >= length {
        eprintln!("stack read error");
        process::exit(0);
    }

    unsafe { _STACK[length - 1 - idx] }
}

fn stack_remove(idx: usize) -> i32 {
    let length = unsafe { _STACK.len() };
    if length == 0 || idx >= length {
        eprintln!("stack read error");
        process::exit(0);
    }

    unsafe{ _STACK.remove(length - 1 - idx) }
}

fn call_stack_push(val: i32) {
    unsafe { _CALL_STACK.push(val); }
}

fn call_stack_pop() -> i32 {
    match unsafe { _CALL_STACK.pop() } {
        Some(n) => {n}
        None => {
            eprintln!("call stack pop error");
            process::exit(0);
        }
    }  
}

fn mem_init(file_name: &String) {
    match fs::read_to_string(file_name) {
        Ok(mut memory) => {
            memory.retain(|x| !x.is_whitespace());
            let mem_vec: Vec<i32> = memory.split(",").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect();

            let mut j = 0;
            while j < mem_vec.len() && j < MEM_SIZE {
                mem_write(j, mem_vec[j]);

                j += 1;
            }
        }
        Err(e) => {
            eprintln!("init memory : {}", e);
            process::exit(0);
        }
    }
}

fn add_stdout(is_int: bool, buf: &mut Vec<String>) {
    let n = stack_pop();
    if is_int {
        buf.push(n.to_string());
    }
    else {
        buf.push(char::from_u32(n as u32).unwrap().to_string());
    }
}

fn do_op(op: fn(i32, i32) -> i32) {
    if unsafe { _STACK.len() } < 2 {
        eprintln!("Error in do_op\n");
        process::exit(0);
    }
    let s0 = stack_pop();
    let s1 = stack_pop();

    stack_push(op(s1, s0));
}

fn cond_jump() -> i32 {
    let jump = stack_pop();

    if stack_pop() == 0 {
        jump
    }
    else {
        0
    }
}

fn load_mem() {
    let idx = stack_pop();

    stack_push(mem_read(idx as usize));
}

fn store_mem() {
    let idx = stack_pop();
    let val = stack_pop();

    mem_write(idx as usize, val);
}

fn copy_stack() {
    let idx = stack_pop();

    let val = stack_read(idx as usize);

    stack_push(val);
}

fn remove_stack() {
    let idx = stack_pop();

    let val = stack_remove(idx as usize);

    stack_push(val);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_len = args.len();
    
    if args_len == 1 {
        eprintln!("./my_hvm [--init <init-mem-filename>] [--trace] <code-filename>\nThe format for the initial memory file is: cell0,cell1,...");
        process::exit(0);
    }

    let mut buf: Vec<String> = Vec::new();

    let mut trace = false;

    // Read argmuments and Initialize memory and a trace variable.
    let mut i = 1;
    while i < args.len() - 1 {
        if args[i].eq("--init") {
            i += 1;
            mem_init(&args[i]);
        }
        else if args[i].eq("--trace") {
            trace = true;
        }
        else {
            eprintln!("invalid argument {}", args[i]);
        }
        i += 1;
    }

    match fs::read_to_string(&args[args.len() - 1]) {
        Ok(codes) => {
            let mut i: i32 = 0;
            let mut is_terminate = false;
            while (i as usize) < codes.len() && i >= 0 && !is_terminate {
                let opcode = codes.as_bytes()[i as usize] as char;

                if trace {
                    eprint!("@{} {} ", i, opcode);
                }

                match opcode {
                    'p' => { add_stdout(true, &mut buf); } 
                    'P' => { add_stdout(false, &mut buf); }
                    '0' => { stack_push(0); }
                    '1' => { stack_push(1); }
                    '2' => { stack_push(2); }
                    '3' => { stack_push(3); }
                    '4' => { stack_push(4); }
                    '5' => { stack_push(5); }
                    '6' => { stack_push(6); }
                    '7' => { stack_push(7); }
                    '8' => { stack_push(8); }
                    '9' => { stack_push(9); }
                    '+' => { do_op(|x, y| x + y); }
                    '-' => { do_op(|x, y| x - y); }
                    '*' => { do_op(|x, y| x * y); }
                    '/' => { do_op(|x, y| x / y); }
                    ':' => { do_op(|x, y| ((x > y) as i32) - ((x < y) as i32)); }
                    'g' => { i += stack_pop(); }
                    '?' => { i += cond_jump(); }
                    'c' => { call_stack_push(i + 1); i = stack_pop() - 1; }
                    '$' => { i = call_stack_pop() - 1; }
                    '<' => { load_mem(); }
                    '>' => { store_mem(); }
                    '^' => { copy_stack(); }
                    'v' => { remove_stack(); }
                    'd' => { stack_pop(); }
                    '!' => { is_terminate = true; }
                    _ => {
                        eprintln!("wrong instructions");
                        process::exit(0);
                    }
                }

                if trace {
                    unsafe { eprint!("{:?}\n", _STACK); }
                }
                i += 1;
            }
            for s in buf {
                print!("{}", s);
            }
            println!("");
        }
        Err(e) => {
            eprintln!("Read code : {}", e);
            process::exit(0);
        }
    }

}
