// https://tokio.rs/docs/getting-started/hello-world/
extern crate tokio;

use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

// think: spawner versus executor; futures are lazy!
// Executors are responsible for scheduling asynchronous tasks, 
// driving them to completion.
fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    // It’s worth reiterating that returned futures are lazy, 
    // i.e., no work is performed when calling the combinator. 
    // Instead, once all the asynchronous steps are sequenced, 
    // the final Future (representing the entire task) is ‘spawned’ 
    // (i.e., run). This is when the previously defined work starts 
    // getting run. In other words, the code we’ve written below
    // does not actually create a TCP stream.

    let client = TcpStream::connect(&addr).and_then(|stream| {
        println!("created stream!");
        // The io::write_all function takes ownership of stream, 
        // returning a Future that completes once the entire message 
        // has been written to the stream.
        //
        // result is a Result that contains the original stream (compare to and_then, 
        // which passes the stream without the Result wrapper.
        io::write_all(stream, "hello world\n").then(|result| {
            println!("wrote to stream; success = {:?}", result.is_ok());
            Ok(())
        })
    })
    .map_err(|err| {
        // All tasks must have an `Error` type of `()`. This forces error
        // handling and helps avoid silencing failures.
        //
        // In our example, we are only going to log the error to STDOUT.
        println!("connection error = {:?}", err);
    });

    println!("About to create the stream and write to ...");
    // we only have a single task running on the executor, so the client 
    // task is the only one blocking run from returning
    tokio::run(client);
    // our Future has been run to completion
    println!("Stream has been created and written to!");
}