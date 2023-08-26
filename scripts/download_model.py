import subprocess
from huggingface_hub import hf_hub_download

TOKEN = "token-here"
subprocess.run(["huggingface-cli", "login", "--token", TOKEN])
hf_hub_download(
    repo_id="TheBloke/Wizard-Vicuna-7B-Uncensored-GGML",
    filename="Wizard-Vicuna-7B-Uncensored.ggmlv3.q8_0.bin",
    local_dir="../backend/models",
)
