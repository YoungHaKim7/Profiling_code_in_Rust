# Profiling_code_in_Rust
- https://github.com/flamegraph-rs/flamegraph

- Install it, and run

```bash
# Rust projects
cargo flamegraph

# Arbitrary binaries
flamegraph -- /path/to/binary
```

# How to interpret a flamegraph? 
- https://www.reddit.com/r/rust/comments/wf9mbn/how_to_interpret_a_flamegraph/?rdt=61382

# C++로 만든 hotspot 
https://github.com/KDAB/hotspot

<hr />



# (221220)Profiling Code in Rust - by Vitaly Bragilevsky - Rust Linz, December 2022 | Rust
- https://youtu.be/JRMOIE_wAFk?si=Ngl2T0FWl74H8v_5
  - 동영상내용 github 코드(https://github.com/bravit/generate_parentheses)

- [(230110) Heap profiling Rust programs with DHAT | chris biscardi](https://youtu.be/AJhKaoyc4pY?si=bkzCTuFCtmY3DNqd)



<hr />

# Rust Profiling 도구 종류
- https://nnethercote.github.io/perf-book/profiling.html

|||
|-|-|
|perf||
|gprof||
|callgrind / <br>cachegrind||
|DTrace||
|Other||

# Profilers(Rust)
- https://nnethercote.github.io/perf-book/profiling.html
  - There are many different profilers available, each with their strengths and weaknesses. The following is an incomplete list of profilers that have been used successfully on Rust programs.

<li><a href="https://perf.wiki.kernel.org/index.php/Main_Page">perf</a> is a general-purpose profiler that uses hardware performance counters.
<a href="https://github.com/KDAB/hotspot">Hotspot</a> and <a href="https://profiler.firefox.com/">Firefox Profiler</a> are good for viewing data recorded by perf.
It works on Linux.</li>
<li><a href="https://developer.apple.com/forums/tags/instruments">Instruments</a> is a general-purpose profiler that comes with Xcode on macOS.</li>
<li><a href="https://www.intel.com/content/www/us/en/developer/tools/oneapi/vtune-profiler.html">Intel VTune Profiler</a> is a general-purpose profiler. It works on Windows,
Linux, and macOS.</li>
<li><a href="https://developer.amd.com/amd-uprof/">AMD μProf</a> is a general-purpose profiler. It works on Windows and Linux.</li>
<li><a href="https://github.com/mstange/samply/">samply</a> is a sampling profiler that produces profiles that can be viewed
in the Firefox Profiler. It works on Mac and Linux.</li>
<li><a href="https://github.com/flamegraph-rs/flamegraph">flamegraph</a> is a Cargo command that uses perf/DTrace to profile your
code and then displays the results in a flame graph. It works on Linux and
all platforms that support DTrace (macOS, FreeBSD, NetBSD, and possibly
Windows).</li>
<li><a href="https://www.valgrind.org/docs/manual/cg-manual.html">Cachegrind</a> &amp; <a href="https://www.valgrind.org/docs/manual/cl-manual.html">Callgrind</a> give global, per-function, and per-source-line
instruction counts and simulated cache and branch prediction data. They work
on Linux and some other Unixes.</li>
<li><a href="https://www.valgrind.org/docs/manual/dh-manual.html">DHAT</a> is good for finding which parts of the code are causing a lot of
allocations, and for giving insight into peak memory usage. It can also be
used to identify hot calls to <code class="hljs">memcpy</code>. It works on Linux and some other
Unixes. <a href="https://github.com/nnethercote/dhat-rs/">dhat-rs</a> is an experimental alternative that is a little less
powerful and requires minor changes to your Rust program, but works on all
platforms.</li>
<li><a href="https://github.com/KDE/heaptrack">heaptrack</a> and <a href="https://github.com/koute/bytehound">bytehound</a> are heap profiling tools. They work on Linux.</li>
<li><a href="https://github.com/nnethercote/counts/"><code class="hljs">counts</code></a> supports ad hoc profiling, which combines the use of <code class="hljs">eprintln!</code>
statement with frequency-based post-processing, which is good for getting
domain-specific insights into parts of your code. It works on all platforms.</li>
<li><a href="https://github.com/plasma-umass/coz">Coz</a> performs <em>causal profiling</em> to measure optimization potential, and has
Rust support via <a href="https://github.com/plasma-umass/coz/tree/master/rust">coz-rs</a>. It works on Linux.</li>
