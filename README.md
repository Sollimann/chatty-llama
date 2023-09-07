<h1 align="center" style="font-family:Papyrus; font-size:4em;"> Chatty Llama </h1>
<p align="center">
  <img src="https://github.com/Sollimann/chatty-llama/blob/main/docs/images/chatty-llama.jpg" width="350" ">
</p>

<p align="center">
    <em>A fullstack chat app utilizing Llama LLMs</em>
</p>

[![minimum rustc 1.60](https://img.shields.io/badge/rustc-1.60+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/Sollimann/chatty-llama/graphs/commit-activity)
[![GitHub pull-requests](https://img.shields.io/github/issues-pr/Sollimann/chatty-llama.svg)](https://GitHub.com/Sollimann/chatty-llama/pulls)
[![GitHub pull-requests closed](https://img.shields.io/github/issues-pr-closed/Sollimann/chatty-llama.svg)](https://GitHub.com/Sollimann/chatty-llama/pulls)
![ViewCount](https://views.whatilearened.today/views/github/Sollimann/chatty-llama.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## How to run

#### 1. Install huggingface-cli


```sh
$ make install-huggingface-cli
```

#### 2. Export huggingface token

Create a huggingface token: https://huggingface.co/settings/tokens

, then set the token as env variable on your machine:

```sh
$ export HF_TOKEN=<your-token-here>
```

#### 3. Download the [Llama-2-7B-Chat-GGML](https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGML) model

```sh
$ make download-model
```

#### 4. Run the chat app

```sh
$ make chatty-llama
```

**Enjoy!**

<p align="center">
  <img src="https://github.com/Sollimann/chatty-llama/blob/main/docs/images/chat.png" width="550" ">
</p>

