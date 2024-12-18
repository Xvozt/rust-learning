## Channels

1. It has 2 parts: transmitter and reciever
2. Trasmitter is an upstream part where you put messages
3. Reciever is a downstream part where message end up
4. You can call method on reciever with the data you want to send
5. And another part of the code checks the recieving end for arriving message
6. A channel is said to be *closed* if one of two parts is dropped

Example:
```rust
let val = String::from("hi");
let (tx, rx) = mpsc::channel();

tx.send(val).unwrap(); // sending
let recieved = rx.recv().unwrap(); // recieving

```

## Mutex
1. Mutex guards the value
2. Before accesing the data we need to call lock
3. The call will block current thread until it can have a lock
4. The lock will fail if another thread that holds lock panicked (so lock returns Result<>)
5. Than use data (lock returns MutexGuard smart pointer that implements Deref)
6. Than unlock mutex for others (it happens automatically using Drop trait)

Example:
```rust
let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
```
