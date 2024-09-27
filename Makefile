clean:
	@rm -rf ./pkg
build:
	wasm-pack build --scope nmemonica --release

pack: build
	wasm-pack pack

test:
#	integration/module tests
#	requires node + npm
	@npm --prefix ./tests install
	@npm run --prefix ./tests test

version:
	rustc --version --verbose
	@echo ""
	cargo --version --verbose
	@echo ""
	wasm-pack --version
	rustup show
	@echo ""