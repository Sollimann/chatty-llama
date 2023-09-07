#!/bin/bash

# Install hf
python3 -m pip install huggingface_hub
echo "export PATH=\"`python3 -m site --user-base`/bin:\$PATH\"" >> ~/.zshrc 
source ~/.zshrc

