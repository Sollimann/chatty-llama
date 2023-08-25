## Backend WebSocket API serving the LLM


### Test from terminal

Install `websocat` command-line client for WebSockets.

```
$ cargo install --feature=ssl websocat
```

Start websocket server

```
$ RUST_LOG=info cargo run
```

Call websocket endpoint

```
$ ➜  ~ websocat ws://127.0.0.1:8080/ws/
halla
Bot: halla
```