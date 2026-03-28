use bm_divan_x1::loops;

fn main() {
    divan::main();
}

#[divan::bench(args = [1_000, 1_000_000])]
fn loops_bench(count: i64) -> i64 {
    loops(count)
}
