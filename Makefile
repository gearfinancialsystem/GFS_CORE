build: $(if $(filter rustlibs,$(MAKECMDGOALS)),build_rustlibs,build_main)

build_main:
	cargo build

build_rustlibs:
	@for lib in libs-rust/*/Cargo.toml; do \
		cd $$(dirname $$lib) && cargo build; \
	done

build_python:
	# need to activate py venv first !
	cd libs/actus-core && maturin develop
	cd libs/lib2 && maturin develop

run:
	cargo run

run-applicability-generator:
	cd app-applicability-generator && . .venv/bin/activate && python app.py

run-concrete-contracts:
	cd app-concrete-contracts && . .venv/bin/activate && python app.py

run-interface-commerciale-app:
	cd app-interface-commerciale && . .venv/bin/activate && python app.py

run-tauri-dev:
	cd app-tauri-main && cargo tauri dev

clean:
	rm -rf target/
