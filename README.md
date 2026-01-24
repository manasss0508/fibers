# Fibers — Minimal Stackful Fiber Runtime in Rust

This repository contains a **minimal implementation of stackful fibers (user-space threads)** written entirely in Rust.  
It demonstrates how **manual context switching and stack switching** can be implemented on x86-64 using Rust nightly and inline assembly.

This project is intended for **learning and experimentation**, not production use.

---

## What Are Fibers?

Fibers (also called user-space threads or stackful coroutines) are lightweight execution units that:

- Run in **user space**
- Have their **own stacks**
- Are **cooperatively scheduled**
- Do **not** rely on OS thread scheduling

Unlike Rust’s `async/await`, fibers are **stackful**, meaning each fiber keeps a full call stack.

---

## Features

- Stackful fibers with dedicated stacks
- Manual CPU context switching
- Cooperative scheduling via explicit `yield`
- Simple round-robin runtime
- No OS threads, no async/await, no external libraries

---

## Project Structure

```
src/
├── main.rs      # Example usage
├── runtime.rs   # Fiber scheduler and runtime
└── thread.rs    # Fiber (thread) and CPU context definitions
```

---

## Core Design

### Fiber (Thread)

Each fiber owns:

- A **fixed-size stack**
- A saved **CPU context**
- A lifecycle state

```rust
pub struct Thread {
    pub stack: Vec<u8>,
    pub ctx: ThreadContext,
    pub state: State,
}
```

---

### CPU Context

The CPU context stores **callee-saved registers** according to the x86-64 System V ABI:

```rust
#[repr(C)]
pub struct ThreadContext {
    pub rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}
```

---

### Runtime (Scheduler)

The runtime manages all fibers and switches execution between them:

```rust
pub struct Runtime {
    threads: Vec<Thread>,
    current: usize,
}
```

Scheduling is cooperative and round-robin on a single OS thread.

---

### Yielding

Fibers explicitly yield control back to the runtime:

```rust
yield_thread();
```

There is **no preemption** — a fiber that never yields will block the runtime.

---

## Example

```rust
fn main() {
    let mut runtime = Runtime::new();
    runtime.init();

    runtime.spwan(|| {
        let id = 1;
        for i in 0..10 {
            println!("thread : {} counter : {}", id, i);
            yield_thread();
        }
    });

    runtime.spwan(|| {
        let id = 2;
        for i in 0..10 {
            println!("thread : {} counter : {}", id, i);
            yield_thread();
        }
    });

    runtime.run();
}
```

---

## Requirements

- x86-64 architecture
- Rust nightly
- Uses `#![feature(naked_functions)]`

Run with:

```bash
cargo +nightly run
```

---

## Safety and Limitations

⚠️ This project uses `unsafe` code intentionally.

- No stack overflow protection
- No preemptive scheduling
- Not production-ready

---

## License

MIT License
