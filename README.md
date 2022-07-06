# iv

imageboard parser library

# usage

```rust
use iv::boards::{dvach::Dvach, fourchan::Fourchan, ImageBoard};

static BOARD: &str = "b";

fn main() {
    // getting 2ch.hk and 4chan.org /b/ last thread
    println!("{:?}", Dvach.get_last_thread(BOARD.to_owned()));
    println!("{:?}", Fourchan.get_last_thread(BOARD.to_owned()));
}
```
