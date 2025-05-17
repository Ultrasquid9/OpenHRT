# lint, then run
default: lint run 

# run
run:
	cargo run

# format
fmt:
	cargo fmt

# format, then run clippy 
lint: fmt
	cargo clippy

# run with flamegraph 
fg: 
	cargo flamegraph --dev
