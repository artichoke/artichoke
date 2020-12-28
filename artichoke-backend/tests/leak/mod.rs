#![allow(dead_code)]

#[cfg(target_os = "linux")]
use std::mem::MaybeUninit;

use artichoke_backend::Artichoke;

#[derive(Debug)]
pub struct Detector<'a> {
    interp: &'a mut Artichoke,
    test: String,
    iterations: usize,
    tolerance: i64, // in bytes
}

impl<'a> Detector<'a> {
    pub fn new<T>(test: T, interp: &'a mut Artichoke) -> Self
    where
        T: Into<String>,
    {
        Self {
            interp,
            test: test.into(),
            iterations: 0,
            tolerance: 0,
        }
    }

    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn with_tolerance(mut self, tolerance: i64) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn check_leaks<F>(self, execute: F)
    where
        F: for<'b> FnMut(&'b mut Artichoke),
    {
        self.check_leaks_with_finalizer(execute, |_| {});
    }

    pub fn check_leaks_with_finalizer<F, G>(self, mut execute: F, finalize: G)
    where
        F: for<'b> FnMut(&'b mut Artichoke),
        G: for<'b> FnOnce(&'b mut Artichoke),
    {
        let start_mem = resident_memsize();
        for _ in 0..self.iterations {
            execute(self.interp);
        }
        finalize(self.interp);
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

#[derive(Debug)]
pub struct Looper {
    test: String,
    iterations: usize,
    tolerance: i64, // in bytes
}

impl Looper {
    pub fn new<T>(test: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            test: test.into(),
            iterations: 0,
            tolerance: 0,
        }
    }

    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn with_tolerance(mut self, tolerance: i64) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn check_leaks<F>(self, execute: F)
    where
        F: FnMut(),
    {
        self.check_leaks_with_finalizer(execute, || {});
    }

    pub fn check_leaks_with_finalizer<F, G>(self, mut execute: F, finalize: G)
    where
        F: FnMut(),
        G: FnOnce(),
    {
        let start_mem = resident_memsize();
        for _ in 0..self.iterations {
            execute();
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

#[cfg(target_os = "linux")]
fn resident_memsize() -> i64 {
    let mut out = MaybeUninit::<libc::rusage>::uninit();
    assert!(unsafe { libc::getrusage(libc::RUSAGE_SELF, out.as_mut_ptr()) } == 0);
    let out = unsafe { out.assume_init() };
    out.ru_maxrss
}

#[cfg(not(target_os = "linux"))]
fn resident_memsize() -> i64 {
    0
}
