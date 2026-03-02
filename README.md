# OS Camp - Rust & OS Advanced Experiments

A Rust advanced and operating system introductory exercise repository in the style of [rustlings](https://github.com/rust-lang/rustlings).
Learn Rust concurrency programming, async programming, `no_std` development, and operating system core concepts through completing code and passing tests.

## Prerequisites

- Rust toolchain (stable, >= 1.75)
- Linux environment: most exercises target x86_64; **Module 4 (context switching) only supports riscv64** and requires a riscv64 environment or QEMU user-mode emulation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Exercise Structure

**6 modules, 23 exercises** in total, from easy to advanced:

### Module 1: Concurrency (Synchronous) — `01_concurrency_sync/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_thread_spawn` | `thread::spawn`, `move` closures, `join` |
| 2 | `02_mutex_counter` | `Arc<Mutex<T>>`, shared state concurrency |
| 3 | `03_channel` | `mpsc::channel`, multiple producer pattern |
| 4 | `04_process_pipe` | `Command`, `Stdio::piped()`, process pipes |

### Module 2: no_std Development — `02_no_std_dev/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_mem_primitives` | `no_std` memory primitives: memcpy, memset, memmove, strlen, strcmp |
| 2 | `02_bump_allocator` | `GlobalAlloc` trait, Bump allocator, CAS-based thread safety |
| 3 | `03_free_list_allocator` | Free-list allocator, intrusive linked list, first-fit strategy |
| 4 | `04_syscall_wrapper` | Cross-arch syscall ABI (x86_64/aarch64/riscv64), inline assembly |
| 5 | `05_fd_table` | File descriptor table, `Arc<dyn File>`, fd reuse strategy |

### Module 3: OS Concurrency Advanced — `03_os_concurrency/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_atomic_counter` | `AtomicU64`, `fetch_add`, CAS loop |
| 2 | `02_atomic_ordering` | Memory ordering, Release-Acquire, `OnceCell` |
| 3 | `03_spinlock` | Spinlock implementation, `compare_exchange`, `spin_loop` |
| 4 | `04_spinlock_guard` | RAII guard, `Deref`/`DerefMut`/`Drop` |
| 5 | `05_rwlock` | Writer-priority read-write lock from scratch (no `std::sync::RwLock`) |

### Module 4: Context Switching — `04_context_switch/` (riscv64 only)

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_stack_coroutine` | Callee-saved registers, stack frames, context switching |
| 2 | `02_green_threads` | Green thread scheduler, cooperative scheduling, yield |

Module 4 only runs on **riscv64**. Run `./check.sh` or use the `oscamp` CLI as with the rest of the repository — no separate scripts needed. See `exercises/04_context_switch/README.md` for details.

### Module 5: Async Programming — `05_async_programming/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_basic_future` | Manual implementation of `Future` trait, `Poll`, `Waker` |
| 2 | `02_tokio_tasks` | `tokio::spawn`, `JoinHandle`, concurrent tasks |
| 3 | `03_async_channel` | `tokio::sync::mpsc`, async producer-consumer |
| 4 | `04_select_timeout` | `tokio::select!`, timeout control, race execution |

### Module 6: Page Tables — `06_page_table/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_pte_flags` | SV39 PTE bit layout, bit operations to construct/parse page table entries |
| 2 | `02_page_table_walk` | Single-level page tables, VPN/offset splitting, address translation, page faults |
| 3 | `03_multi_level_pt` | SV39 three-level page tables, page table walk, huge pages (2MB) mapping |
| 4 | `04_tlb_sim` | TLB lookup/insert/FIFO replacement, flush (all/by page/by ASID), MMU simulation |

## Quick Start

```bash
# 1. Clone repository
git clone <repo-url> && cd oscamp-base-experiment

# 2. Build interactive CLI tool
cargo build -p oscamp-cli

# 3. Start interactive exercise mode (recommended)
./target/debug/oscamp watch
```

## Interactive CLI Tool (`oscamp`)

Built-in interactive terminal tool similar to rustlings, supporting real-time file watching and progress tracking:

```bash
oscamp              # Start interactive watch mode (default)
oscamp watch        # Same as above
oscamp list         # View completion status of all exercises
oscamp check        # Check all exercises in batch
oscamp run <pkg>    # Run tests for specified exercise
oscamp hint <pkg>   # View exercise hint
oscamp help         # Show help
```

### Watch Mode Features

- **Automatic file change detection**: Automatically re-run tests after saving files
- **Auto-jump**: Automatically jump to next unfinished exercise after current one passes
- **Real-time progress bar**: Show overall completion progress
- **Shortcuts**:
  - `h` — View hint for current exercise
  - `l` — View list of all exercises
  - `n` / `p` — Next / Previous exercise
  - `r` / `Enter` — Re-run tests
  - `q` / `Esc` — Quit

### Manual Execution

```bash
# Run tests for a specific exercise
cargo test -p thread_spawn

# View detailed output
cargo test -p thread_spawn -- --nocapture

# Check all exercises
cargo test --workspace
```

## Workflow

1. **Start**: Run `./target/debug/oscamp watch` to enter interactive mode
2. **Read**: Open current exercise file `src/lib.rs`, read documentation to understand concepts
3. **Code**: Find `todo!()` markers, complete code according to comment hints
4. **Save**: After saving file, CLI automatically re-runs tests
5. **Pass**: After passing tests, automatically jump to next exercise; press `h` to view hints anytime

## Notes

- Some exercises (e.g., Module 2 syscall wrapper, Module 4 assembly) require a **Linux** environment; Module 4 only supports **riscv64**
- It is recommended to complete exercises in module order; within each module, exercises progress from easy to advanced

## License

MIT
