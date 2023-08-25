#!/bin/bash

echo "Downloading 30B LLM model..."
rm -rf ../backend/models/
curl -sSL https://g.bodaay.io/hfd

hfdownloader -m TheBloke/Wizard-Vicuna-30B-Uncensored.ggmlv3.q8_0 -s ../backend/models/
