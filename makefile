build:
	cargo build --bin dijkstra --bin dial --bin radixheap --release

experiments:
	cargo build --example experiment_1 --example experiment_2 --release