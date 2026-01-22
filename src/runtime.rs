#![feature(naked_functions)]
use std::arch::*;
use std::thread::sleep;
use crate::thread::*;


const DEFAULT_STACK_SIZE : usize = 1024 * 1024 * 2; //2mb = 2097152KB
const max_thread: usize = 4;
static mut RUNTIME: usize = 0; // pointer to runtime



// struct : that represent our thread scheduling runtime
pub struct Runtime {
    threads: Vec<Thread>, // vector that contains all threads
    current: usize, // current running thread position in vector

}

impl Runtime {

    // create new runtime
    pub fn new() -> Self {
        let base_thread = Thread {
            stack: vec![0_u8;DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Running
        };// this thread will be used to save context of main thread

        let mut threads = vec![base_thread];

        for i in 1..max_thread {
            threads.push(Thread::new());

        }

        Runtime {
            threads,
            current: 0,
        }
    }

    // initialze RUNTIME pointer of lineNO-6
    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;

        }
    }

    // assign function to AVAIABLE thread
    // mark thread as READY
    pub fn spwan(&mut self, f: fn()) {
        let  available = self.threads.iter_mut()
            .find(|t|t.state == State::Available)
            .expect("no thread available");

        let size = available.stack.len();

        unsafe {
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);
            let s_ptr = (s_ptr as usize & !15) as *mut u8;
            std::ptr::write(s_ptr.offset(-16) as *mut u64, guard as u64);
            std::ptr::write(s_ptr.offset(-24) as *mut u64, skip as u64);
            std::ptr::write(s_ptr.offset(-32) as *mut u64, f as u64);
            available.ctx.rsp = s_ptr.offset(-32) as u64;
        }

        available.state = State::Ready;
    }


    // this function is implemented to tranfer control
    // from one thread to another by utilizing switch function
    // check next READY thread -> mark current RUNNING Thread READY
    // -> mark finded READY thread as RUNNING -> call SWITCH function
    fn t_yield(&mut self) -> bool {
        let mut pos = self.current;

        // to check next thread that is in READY state
        // READY : state that indicate
        while self.threads[pos].state != State::Ready {
            pos += 1; // 1, 2, 3

            // if pos at last element means no thread is READY
            if pos == self.threads.len() { // at pos == 4
                pos = 0;
            }

            //if pos at start element mean no thread is READY
            if pos == self.current { // at index 0
                return false;
            }
        }

        // as current thread is RUNNING
        // we mark current RUNNING thread as READY
        // beacuse we need to resume proccessing this thread in future
        if self.threads[self.current].state != State::Available {
            self.threads[self.current].state = State::Ready
        }

        // we have found next READY thread
        //then we will mark it as RUNNING
        self.threads[pos].state = State::Running;
        let old_pos = self.current; // pos of last RUNNING thread
        self.current = pos;

        unsafe {
            let old : *mut ThreadContext = &mut self.threads[old_pos].ctx;
            let new: *const ThreadContext = & self.threads[pos].ctx;
            asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
        }
        // rdi -> context of RUNNING thread
        // rsi -> context of READY thread we are going start

        //
        self.threads.len() > 0

    }

    // start runtime it will continously call t_yield() until it return false
    pub fn run(&mut self) {
        while self.t_yield() {
            // nothing
        }
        std::process::exit(0);
    }

    // This is the return function that we call when a thread is finished
    fn t_return(&mut self) {
        if self.current != 0 { //if current thread is not base_thread
            self.threads[self.current].state = State::Available;
            // thread ended we set is state to available stating runtime
            // thread is available for new task
            self.t_yield();
        }
    }
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn switch() {
    naked_asm!(
    // saving register context struct
    "MOV [rdi + 0x00], rsp ",
    "MOV [rdi + 0x08], r15",
    "MOV [rdi + 0x10], r14",
    "MOV [rdi + 0x18], r13",
    "MOV [rdi + 0x20], r12",
    "MOV [rdi + 0x28], rbx",
    "MOV [rdi + 0x30], rbp",
    // moving register from context struct to register
    "MOV rsp, [rsi + 0x00]",
    "MOV r15, [rsi + 0x08]",
    "MOV r14, [rsi + 0x10]",
    "MOV r13, [rsi + 0x18]",
    "MOV r12, [rsi + 0x20]",
    "MOV rbx, [rsi + 0x28]",
    "MOV rbp, [rsi + 0x30]",
    "ret",
    )
}


//
fn guard() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_return();
    }
}

//
#[unsafe(naked)]
unsafe extern "C" fn skip() {
    naked_asm!("ret");
}

//
pub fn yield_thread(){
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    }
}


