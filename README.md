<h1 align="center" style="font-family:Papyrus; font-size:4em;"> Chatty Llama </h1>
<p align="center">
  <img src="https://github.com/Sollimann/chatty-llama/blob/main/docs/images/chatty-llama.jpg" width="350" ">
</p>

<p align="center">
    <em>A fullstack chat app utilizing Llama LLMs</em>
</p>

<p align="center">
    <a href="https://rust-lang.github.io/rfcs/2495-min-rust-version.html"><img src="https://img.shields.io/badge/rustc-1.60+-blue.svg" alt="minimum rustc 1.60"></a>
    <a href="https://GitHub.com/Sollimann/chatty-llama/graphs/commit-activity"><img src="https://img.shields.io/badge/Maintained%3F-yes-green.svg" alt="Maintenance"></a>
    <a href="https://GitHub.com/Sollimann/chatty-llama/pulls"><img src="https://img.shields.io/github/issues-pr/Sollimann/chatty-llama.svg" alt="GitHub pull-requests"></a>
    <a href="https://GitHub.com/Sollimann/chatty-llama/pulls"><img src="https://img.shields.io/github/issues-pr-closed/Sollimann/chatty-llama.svg" alt="GitHub pull-requests closed"></a>
    <img src="https://views.whatilearened.today/views/github/Sollimann/chatty-llama.svg" alt="ViewCount">
    <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
</p>

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

**PS!** If you're having issues connecting to the `backend`, try running `make chatty-llama-host` instead.

In your browser, open http://localhost:80

**Enjoy!**

<p align="center">
  <img src="https://github.com/Sollimann/chatty-llama/blob/main/docs/images/chat.png" width="550" ">
</p>
