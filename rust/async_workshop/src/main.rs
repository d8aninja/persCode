//basics: need yield and notify primitives to think about async
// async is not concurrency or parallelism
// all std libs are synchronus
// what is async?
// a future is like a loose stack frame in the call stack; "wandering cat"; loose memory(?)
// pin means the stack frame has a stable memory location
// futures are polling driven
// rust doesn't have a runtime; and you need one in order to run asynchronous (tokio)
// you need something component driving all this stuff to completion
// keywords give us hooks to the future types
// for loop is just sugar for the iterator trait
// future to asyn is like for loop to iterator/
// all in one async runtime for asyn apps in rust
// important apis
// tokio::main is a procedural macro (new)
// tokio::spawn takes a future.  stanard thread spawner but asyncronous.  spawns a tasks on the tokio runtime.  output type must be unit
// background processes are daemon tasks
// tokio::sync::{mpsc, Mutex}
// mutual exclusion block lets you take some data and share it across threads
// remember only one thread can access data at any time
// block is an asyn function
// mpmc is really hard (so, singular consumer here)
// channels in general are for SENDING an item between threads (NOT sharing)
// rust is affine typed, data can't be used again after its moved
// clones, or locks.  queues in locks.  to consume / read we look at the top of the queue
// no tokio r/w lock YET
// futures has a bilock (two halves, one reader one writer)
// MUTEX is for "multiple" readers
// futures03 is the current version, but in a preview state
// lots of people on futures01 since they don't want to be on nightly
// futures02 is dep
// a stream is an async iterator and looks a lot like an iterator.  next method, returns the option of an item
// stream is opposite of sync
use std::net::{SocketAddr};
use std::error::Error;







pub trait Future {
    type Output;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_> // waker, leaf / children futures of the parent (has to know what to wake up)
    ) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending
}