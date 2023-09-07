install-huggingface-cli:
	./scripts/install_hf.sh

download-model:
	python3 scripts/download_model.py

chatty-llama:
	docker compose up -d
