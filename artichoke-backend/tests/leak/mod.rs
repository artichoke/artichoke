use std::convert::AsRef;
use std::mem;

pub struct Detector {
    test: String,
    iterations: usize,
    tolerance: i64, // in bytes
}

impl Detector {
    pub fn new<T: AsRef<str>>(test: T, iterations: usize, tolerance: i64) -> Self {
        Self {
            test: test.as_ref().to_owned(),
            iterations,
            tolerance,
        }
    }

    pub fn check_leaks<F>(&self, execute: F)
    where
        F: FnMut(usize) -> (),
    {
        self.check_leaks_with_finalizer(execute, || {});
    }

    pub fn check_leaks_with_finalizer<F, G>(&self, mut execute: F, finalize: G)
    where
        F: FnMut(usize) -> (),
        G: FnOnce() -> (),
    {
        let start_mem = resident_memsize();
        for i in 0..self.iterations {
            execute(i);
        }
        finalize();
        let end_mem = resident_memsize();
        assert!(
            end_mem <= start_mem + self.tolerance,
            "Plausible memory leak in test {}!\nAfter {} iterations, usage before: {}, usage after: {}",
            self.test,
            self.iterations,
            start_mem,
            end_mem
        );
    }
}

fn resident_memsize() -> i64 {
    let mut out: libc::rusage = unsafe { mem::zeroed() };
    assert!(unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut out) } == 0);
    out.ru_maxrss
}
