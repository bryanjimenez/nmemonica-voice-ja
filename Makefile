clean:
	@rm -rf ./pkg
build:
	wasm-pack build --scope nmemonica --release

pack: build
	wasm-pack pack

test:
	node ./tests/wasm_js_test.js

version:
	rustc --version --verbose
	@echo ""
	cargo --version --verbose
	@echo ""
	wasm-pack --version
	rustup show
	@echo ""