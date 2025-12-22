// A minimal single-threaded shim for rayon-core
// preventing WASI panics by never spawning threads.

// Basically what we're doing is keep the interface of rayon-core,
// but we just call anything inline instead of dispatching to threads.
// All the scheduling stuff, thread pools, etc. are no-ops or stubs.
// The operations are still being called, but just inline on the main thread.

// Double check that the version of rayon-core matches what Polars expects.
// In Cargo.toml


use std::fmt;

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(_: ThreadPoolBuilder) -> Result<Self, ThreadPoolBuildError> {
        Ok(Self)
    }
    pub fn install<OP, R>(&self, op: OP) -> R
    where OP: FnOnce() -> R + Send, R: Send {
        op() // No thread dispatching, just run directly inline
    }
    pub fn current_num_threads(&self) -> usize { 1 }
}

pub struct ThreadPoolBuilder;

impl ThreadPoolBuilder {
    pub fn new() -> Self { Self }
    pub fn num_threads(self, _: usize) -> Self { self }
    pub fn build(self) -> Result<ThreadPool, ThreadPoolBuildError> {
        Ok(ThreadPool)
    }
    pub fn build_global(self) -> Result<(), ThreadPoolBuildError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct ThreadPoolBuildError;

impl fmt::Display for ThreadPoolBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ThreadPoolBuildError")
    }
}

impl std::error::Error for ThreadPoolBuildError {}

// `join` is the core function in Rayon that Polars uses for parallelism.
// This "implementation" just runs the two operations sequentially.
pub fn join<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
where A: FnOnce() -> RA + Send, B: FnOnce() -> RB + Send, RA: Send, RB: Send {
    (oper_a(), oper_b())
}

pub fn join_context<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
where A: FnOnce(FnContext) -> RA + Send, B: FnOnce(FnContext) -> RB + Send, RA: Send, RB: Send {
    (oper_a(FnContext), oper_b(FnContext))
}

pub fn scope<'scope, OP, R>(op: OP) -> R
where OP: FnOnce(&Scope<'scope>) -> R + Send, R: Send {
    let scope = Scope(std::marker::PhantomData);
    op(&scope)
}

pub fn in_place_scope<'scope, OP, R>(op: OP) -> R
where OP: FnOnce(&Scope<'scope>) -> R {
    let scope = Scope(std::marker::PhantomData);
    op(&scope)
}

pub fn scope_fifo<'scope, OP, R>(op: OP) -> R
where OP: FnOnce(&ScopeFifo<'scope>) -> R + Send, R: Send {
    let scope = ScopeFifo(std::marker::PhantomData);
    op(&scope)
}

pub fn in_place_scope_fifo<'scope, OP, R>(op: OP) -> R
where OP: FnOnce(&ScopeFifo<'scope>) -> R {
    let scope = ScopeFifo(std::marker::PhantomData);
    op(&scope)
}

pub struct Scope<'scope>(std::marker::PhantomData<&'scope ()>);

impl<'scope> Scope<'scope> {
    pub fn spawn<BODY>(&self, body: BODY)
    where BODY: FnOnce(&Scope<'scope>) + Send + 'scope {
        body(self); // Also here, call body directly instead of queuing to thread pool
    }

    pub fn spawn_fifo<BODY>(&self, body: BODY)
    where BODY: FnOnce(&Scope<'scope>) + Send + 'scope {
        body(self);
    }
}

pub struct ScopeFifo<'scope>(std::marker::PhantomData<&'scope ()>);

impl<'scope> ScopeFifo<'scope> {
    pub fn spawn<BODY>(&self, body: BODY)
    where BODY: FnOnce(&ScopeFifo<'scope>) + Send + 'scope {
        body(self);
    }

    pub fn spawn_fifo<BODY>(&self, body: BODY)
    where BODY: FnOnce(&ScopeFifo<'scope>) + Send + 'scope {
        body(self);
    }
}

pub fn spawn<BODY>(_body: BODY)
where BODY: FnOnce() + Send + 'static {
    // In single-threaded mode, we can't spawn, so we just run it
    _body();
}

pub fn spawn_fifo<BODY>(_body: BODY)
where BODY: FnOnce() + Send + 'static {
    // In single-threaded mode, we can't spawn, so we just run it
    _body();
}

pub fn broadcast<OP, R>(_op: OP) -> R
where OP: Fn(BroadcastContext) -> R + Sync, R: Send {
    _op(BroadcastContext { index: 0, num_threads: 1 })
}

pub fn spawn_broadcast<OP, R>(_op: OP) -> R
where OP: Fn(BroadcastContext) -> R + Send + Sync + 'static, R: Send + 'static {
    _op(BroadcastContext { index: 0, num_threads: 1 })
}

#[derive(Debug, Clone, Copy)]
pub struct BroadcastContext {
    pub index: usize,
    pub num_threads: usize,
}

impl BroadcastContext {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn num_threads(&self) -> usize {
        self.num_threads
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FnContext;

impl FnContext {
    pub fn migrated(self) -> bool {
        false  // Never migrated in single-threaded mode
    }
}

pub struct ThreadBuilder;

pub fn current_num_threads() -> usize {
    1
}

pub fn current_thread_index() -> Option<usize> {
    Some(0)
}

pub fn max_num_threads() -> usize {
    1
}

pub fn yield_now() {
    // No-op in single-threaded mode
    // This is ok because this is a scheduling hint, 
    // and we are only running one thread.
}

pub fn yield_local() {
    // No-op in single-threaded mode
    // Also this is a scheduling hint.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yield {
    Executed,
    Idle,
}

// If Polars complains about missing items, simply add empty public structs/fns here.