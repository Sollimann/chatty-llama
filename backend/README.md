## Backend WebSocket API serving the LLM


### Test from terminal

Install `websocat` command-line client for WebSockets.

```
$ cargo install --feature=ssl websocat
```

Start websocket server

**Note!**
* Without the target-cpu=native flag, the generated code aims to be compatible with a wide range of CPUs of the target architecture.
* With the target-cpu=native flag, the generated code can use special instructions available only on your specific CPU model, which can lead to more efficient code for that specific CPU but might not run on older or different models of the same architecture.

```
$ RUST_LOG=info RUSTFLAGS="-C target-cpu=native" cargo run --release
```


Call websocket endpoint

```
$ ➜  ~ websocat ws://127.0.0.1:8080/ws/
halla
Bot: halla
```
