# dhat 사용법(valgrind)
- https://valgrind.org/docs/manual/dh-manual.html

```bash
$ valgrind --tool=dhat ./program
```

<hr />

# Heap profiling Rust programs with DHAT | chris biscardi
- https://youtu.be/AJhKaoyc4pY?si=dHJVZaSoGG03TEe6

<hr />

# Heap profiling and ad hoc profiling for Rust programs. 
- https://github.com/nnethercote/dhat-rs


# rs

- src/main.rs

```rs
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler_ = dhat::Profiler::new_heap();

    println!("Hello, world dhat-rs");
}
```

- Cargo.toml

```toml
[dependencies]
dhat = "0.3.3"

[profile.release]
debug =  1

[features]
dhat-heap = []
```

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

