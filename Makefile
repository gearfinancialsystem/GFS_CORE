build: 
	cargo build

build_python:
	# need to activate py venv first !
	cd libs/actus-core && maturin develop
	cd libs/lib2 && maturin develop

run:
	cargo run

clean:
	rm -rf target/