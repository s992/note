clean:
	cargo clean

build: clean
	cargo build

install: build
	cargo install --path . --force
