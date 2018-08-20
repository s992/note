clean:
	cargo clean

build: clean
	cargo build

install: clean
	cargo install --path . --force
