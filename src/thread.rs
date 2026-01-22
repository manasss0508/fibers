
const DEFAULT_STACK_SIZE : usize = 1024 * 1024 * 2; //2mb = 2097152Byte

// struct : to save cpu register value
#[derive(Debug,Default)]
#[repr(C)]
pub struct ThreadContext {
    pub rsp: u64,//offset 0
    r15: u64,//offset 8
    r14: u64,//offset 10
    r13: u64,//offset 18
    r12: u64,//offset 20
    rbx: u64,//offset 28
    rbp: u64//offset 30
}

//enum : to specify state for thread
#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Available,
    Running,
    Ready,
}

//struct : that represent thread
pub struct Thread {
    pub stack: Vec<u8>, // stack of the thread
    pub ctx: ThreadContext, // saved register of thread
    pub state: State //current state of thread
}

 impl Thread {
   pub  fn new() -> Self {
        Thread {
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Available
        }
    }
}


