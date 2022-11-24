use crate::timing;

pub const RUNS: usize = 100;

pub fn do_benchmark(j: fn() -> usize) -> timing::TimingResult {
    let min = (0..RUNS)
        .map(|_| {
            let timing = timing::Timing::new();
            j();
            return timing.result().get_guration();
        })
        .min()
        .unwrap();
    return timing::TimingResult::from_duration(min);
}
