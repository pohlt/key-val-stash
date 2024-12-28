.PHONY: watch clean debug release run container

watch:
	cargo watch -x run

debug: target/debug/key-val-stash
	cargo build

release: target/release/key-val-stash
	cargo build --release

run: target/debug/key-val-stash
	cargo run

container:
	podman build -t key-val-stash .

run-container:
	podman run -p 28536:28536/udp -v ./database.cbor:/database.cbor key-val-stash
