# json 파일 여기서 로드하면 이쁘게 보임

- https://nnethercote.github.io/dh_view/dh_view.html

# Result

```bash
$ cargo r --features dhat-heap

Hello, world dhat-rs
dhat: Total:     1,088 bytes in 2 blocks
dhat: At t-gmax: 1,088 bytes in 2 blocks
dhat: At t-end:  1,088 bytes in 2 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
```

- 사이트에서 봤을때
  - https://nnethercote.github.io/dh_view/dh_view.html

```
Invocation {
  Mode:    rust-heap
  Command: target/debug/a01_dhat_heap
  PID:     9590
}

Times {
  t-gmax: 265 µs (88.33% of program duration)
  t-end:  300 µs
}

▼ PP 1/1 (2 children) {
    Total:     1,088 bytes (100%, 3,626,666.67/s) in 2 blocks (100%, 6,666.67/s), avg size 544 bytes, avg lifetime 60.5 µs (20.17% of program duration)
    At t-gmax: 1,088 bytes (100%) in 2 blocks (100%), avg size 544 bytes
    At t-end:  1,088 bytes (100%) in 2 blocks (100%), avg size 544 bytes
    Allocated at {
      #0: [root]
    }
  }
  ├── PP 1.1/2 {
  │     Total:     1,024 bytes (94.12%, 3,413,333.33/s) in 1 blocks (50%, 3,333.33/s), avg size 1,024 bytes, avg lifetime 86 µs (28.67% of program duration)
  │     Max:       1,024 bytes in 1 blocks, avg size 1,024 bytes
  │     At t-gmax: 1,024 bytes (94.12%) in 1 blocks (50%), avg size 1,024 bytes
  │     At t-end:  1,024 bytes (94.12%) in 1 blocks (50%), avg size 1,024 bytes
  │     Allocated at {
  │       #1: 0x102971280: <alloc::alloc::Global as core::alloc::Allocator>::allocate (alloc/src/alloc.rs:254:9)
  │       #2: 0x102971280: alloc::raw_vec::RawVecInner<A>::try_allocate_in (alloc/src/raw_vec.rs:472:41)
  │       #3: 0x102971280: alloc::raw_vec::RawVecInner<A>::with_capacity_in (alloc/src/raw_vec.rs:418:15)
  │       #4: 0x102971280: alloc::raw_vec::RawVec<T,A>::with_capacity_in (alloc/src/raw_vec.rs:186:20)
  │       #5: 0x102971280: alloc::vec::Vec<T,A>::with_capacity_in (src/vec/mod.rs:803:20)
  │       #6: 0x102971280: alloc::vec::Vec<T>::with_capacity (src/vec/mod.rs:483:9)
  │       #7: 0x102971280: std::io::buffered::bufwriter::BufWriter<W>::with_capacity (io/buffered/bufwriter.rs:122:33)
  │       #8: 0x102971280: std::io::buffered::linewriter::LineWriter<W>::with_capacity (io/buffered/linewriter.rs:110:29)
  │       #9: 0x102971280: std::io::buffered::linewriter::LineWriter<W>::new (io/buffered/linewriter.rs:90:9)
  │       #10: 0x102971280: std::io::stdio::stdout::{{closure}} (src/io/stdio.rs:677:61)
  │       #11: 0x102971280: std::sync::once_lock::OnceLock<T>::get_or_init::{{closure}} (src/sync/once_lock.rs:310:50)
  │       #12: 0x102971280: std::sync::once_lock::OnceLock<T>::initialize::{{closure}} (src/sync/once_lock.rs:518:19)
  │       #13: 0x102971280: std::sync::poison::once::Once::call_once_force::{{closure}} (sync/poison/once.rs:214:40)
  │       #14: 0x10298e5c4: std::sys::sync::once::queue::Once::call (sync/once/queue.rs:219:21)
  │       #15: 0x10298e054: std::sync::poison::once::Once::call_once_force (sync/poison/once.rs:214:9)
  │       #16: 0x10298e054: std::sync::once_lock::OnceLock<T>::initialize (src/sync/once_lock.rs:517:9)
  │       #17: 0x10296ee38: std::sync::once_lock::OnceLock<T>::get_or_try_init (src/sync/once_lock.rs:396:9)
  │       #18: 0x10296ee38: std::sync::once_lock::OnceLock<T>::get_or_init (src/sync/once_lock.rs:310:15)
  │       #19: 0x10296ee38: std::io::stdio::stdout (src/io/stdio.rs:677:14)
  │       #20: 0x10296ee38: std::io::stdio::print_to (src/io/stdio.rs:1122:21)
  │       #21: 0x10296ee38: std::io::stdio::_print (src/io/stdio.rs:1233:5)
  │       #22: 0x102884bcc: a01_dhat_heap::main (a01_dhat_heap/src/main.rs:9:5)
  │     }
  │   }
  └── PP 1.2/2 {
        Total:     64 bytes (5.88%, 213,333.33/s) in 1 blocks (50%, 3,333.33/s), avg size 64 bytes, avg lifetime 35 µs (11.67% of program duration)
        Max:       64 bytes in 1 blocks, avg size 64 bytes
        At t-gmax: 64 bytes (5.88%) in 1 blocks (50%), avg size 64 bytes
        At t-end:  64 bytes (5.88%) in 1 blocks (50%), avg size 64 bytes
        Allocated at {
          #1: 0x10298e328: alloc::alloc::exchange_malloc (alloc/src/alloc.rs:349:18)
          #2: 0x10298e328: alloc::boxed::Box<T>::new (alloc/src/boxed.rs:274:16)
          #3: 0x10298e328: alloc::boxed::Box<T>::pin (alloc/src/boxed.rs:335:9)
          #4: 0x10298e328: std::sys::sync::mutex::pthread::Mutex::get::{{closure}} (sync/mutex/pthread.rs:23:27)
          #5: 0x10298e328: std::sys::sync::once_box::OnceBox<T>::initialize (sys/sync/once_box.rs:63:72)
          #6: 0x10296e814: std::sys::sync::once_box::OnceBox<T>::get_or_init (sys/sync/once_box.rs:51:21)
          #7: 0x10296e814: std::sys::sync::mutex::pthread::Mutex::get (sync/mutex/pthread.rs:22:9)
          #8: 0x10296e814: std::sys::sync::mutex::pthread::Mutex::lock (sync/mutex/pthread.rs:34:23)
          #9: 0x10296e814: std::sync::reentrant_lock::ReentrantLock<T>::lock (src/sync/reentrant_lock.rs:289:28)
          #10: 0x10296e814: std::io::stdio::Stderr::lock (src/io/stdio.rs:959:29)
          #11: 0x10296e35c: <&std::io::stdio::Stdout as std::io::Write>::write_fmt (src/io/stdio.rs:792:9)
          #12: 0x10296ee20: <std::io::stdio::Stdout as std::io::Write>::write_fmt (src/io/stdio.rs:766:9)
          #13: 0x10296ee20: std::io::stdio::print_to (src/io/stdio.rs:1122:21)
          #14: 0x10296ee20: std::io::stdio::_print (src/io/stdio.rs:1233:5)
          #15: 0x102884bcc: a01_dhat_heap::main (a01_dhat_heap/src/main.rs:9:5)
        }
      }

PP significance threshold: total >= 0.02 blocks (1%)
```
