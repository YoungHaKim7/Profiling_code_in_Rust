# Result

```bash
$ valgrind --tool=dhat ./c01_linux_advent_code_test_valgrind < ../../input/input.txt 

==13862== DHAT, a dynamic heap analysis tool
==13862== Copyright (C) 2010-2024, and GNU GPL'd, by Mozilla Foundation et al.
==13862== Using Valgrind-3.25.0.GIT and LibVEX; rerun with -h for copyright info
==13862== Command: ./c01_linux_advent_code_test_valgrind
==13862== 
dhat_heap ~~~~
constellations: 327
==13862== 
==13862== Total:     370,568 bytes in 3,040 blocks
==13862== At t-gmax: 139,664 bytes in 333 blocks
==13862== At t-end:  8,192 bytes in 1 blocks
==13862== Reads:     6,159,001,163 bytes
==13862== Writes:    372,010 bytes
==13862== 
==13862== To view the resulting profile, open
==13862==   file:///usr/local/libexec/valgrind/dh_view.html
==13862== in a web browser, click on "Load...", and then select the file
==13862==   /home/g/my_project/Rust_Lang/9999/Profiling_code_in_Rust/001_dhat-rs/c01_linux_advent_code_test_valgrind/target/release/dhat.out.13862
==13862== The text at the bottom explains the abbreviations used in the output.


```

