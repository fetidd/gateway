test:
	cargo build && cp ../target/debug/gw_api ./apitest/ && cd ./apitest && uv run ./apitest.py --file ./gw_api --test_path ./tests.yaml && rm ./gw_api
