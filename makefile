build:
	cargo run -- -d -f ./.test.toml build

clean:
	rm -rf build/
	make build

serve: build
	cargo run -- -d -f ./.test.toml serve

release:
	cargo build --release