use std::{
    fs,
    io::{self, Read},
};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler_ = dhat::Profiler::new_heap();

    let file = fs::read_to_string("../input/input.txt");
    println!("dhat_heap : {}", part1(&points));
}
