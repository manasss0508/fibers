use crate::runtime::*;

mod runtime;
mod thread;

fn main() {
    // create runtime
    let mut runtime = runtime::Runtime::new();

    // initalize pointer
    runtime.init();

    // assign task to AVAILABLE thread
    runtime.spwan(|| {
        let id = 1;
        for i in 0..10 {
            println!("thread : {} counter : {}", id, i);
           yield_thread();
        }

    });

    // assign task to AVAILABLE thread
    runtime.spwan(|| {
        let id = 2;
        for i in 0..10 {
            println!("thread : {} counter : {}", id, i);
            yield_thread();
        }
    });

    runtime.run();

}