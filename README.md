## Pipeviewer Rust System Programming

Rust guarantees memory and thread safety at compile-time, 
yet uses zero-cost abstractions without the runtime overhead of a garbage collector.

#### Including:
- Use multithreading to unlock the power of multiple cores
- Get to know data-flow rate and speed through a pipeline
- Display time-based statistics using stderr
- Build the middleware project to control the flow of data between two processes
- Set up a project for success
- Test and publish the project on crates .io

##### Generate a 128k file with random data
```
dd if=/dev/urandom bs=1024 count=128 of=myfile
cat myfile | target/debug/pipeviewer > myfile2
```
