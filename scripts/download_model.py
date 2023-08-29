import subprocess
from huggingface_hub import hf_hub_download

TOKEN = "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"
subprocess.run(["huggingface-cli", "login", "--token", TOKEN])

# https://huggingface.co/TheBloke/Llama-2-13B-chat-GGML
hf_hub_download(
    repo_id="TheBloke/Llama-2-7B-chat-GGML",
    filename="llama-2-7b-chat.ggmlv3.q2_K.bin",
    local_dir="../backend/models",
)
