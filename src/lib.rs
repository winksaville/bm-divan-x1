use std::hint::black_box;

pub fn loops(count: i64) -> i64 {
    let mut i: i64 = 0;
    for _ in 0..count {
        black_box(i += 1);
    }

    i
}
