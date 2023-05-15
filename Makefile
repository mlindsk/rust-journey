primitives:
	cargo run --bin primitives

structs:
	cargo run --bin structs

containers:
	cargo run --bin containers

controlflow:
	cargo run --bin controlflow

readme_render:
	quarto render README.qmd

readme_preview:
	quarto preview README.qmd
