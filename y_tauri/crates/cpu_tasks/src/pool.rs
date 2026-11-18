use rayon::{ThreadPoolBuildError, ThreadPoolBuilder};

pub fn available_threads() -> usize {
    rayon::current_num_threads()
}

pub fn run_with_threads<T, F>(threads: usize, op: F) -> Result<T, ThreadPoolBuildError>
where
    F: FnOnce() -> T + Send,
    T: Send,
{
    let pool = ThreadPoolBuilder::new().num_threads(threads).build()?;
    Ok(pool.install(op))
}

pub fn thread_steps(max: usize) -> Vec<usize> {
    let mut steps: Vec<usize> = std::iter::successors(Some(1), |&n| Some(n * 2))
        .take_while(|&n| n < max)
        .collect();
    steps.push(max.max(1));
    steps
}
