```
➜  copy-2016-07-03 git:(master) ✗ cargo build             
warning: unused manifest key: package.edition
   Compiling copy1 v0.1.0 (file:///home/u2/cryptape/test_copy/copy-2016-07-03)
src/main.rs:6:1: 6:23 error: the trait bound `[i32; 100]: std::clone::Clone` is not satisfied [E0277]
src/main.rs:6 #[derive(Copy, Clone)]
              ^~~~~~~~~~~~~~~~~~~~~~
src/main.rs:6:1: 6:23 note: in this expansion of #[derive_Clone] (defined in src/main.rs)
src/main.rs:6:1: 6:23 help: run `rustc --explain E0277` to see a detailed explanation
src/main.rs:6:1: 6:23 help: the following implementations were found:
src/main.rs:6:1: 6:23 help:   <[T; 0] as std::clone::Clone>
src/main.rs:6:1: 6:23 help:   <[T; 1] as std::clone::Clone>
src/main.rs:6:1: 6:23 help:   <[T; 2] as std::clone::Clone>
src/main.rs:6:1: 6:23 help:   <[T; 3] as std::clone::Clone>
src/main.rs:6:1: 6:23 help: and 29 others
src/main.rs:6:1: 6:23 note: required by `std::clone::assert_receiver_is_clone`
error: aborting due to previous error
error: Could not compile `copy1`.

To learn more, run the command again with --verbose.
```


```
➜  copy-nightly-2017-12-05 git:(master) ✗ cargo build               
warning: unused manifest key: package.edition
   Compiling copy1 v0.1.0 (file:///home/u2/cryptape/test_copy/copy-nightly-2017-12-05)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35 secs
```
