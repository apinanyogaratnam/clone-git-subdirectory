VERSION := 0.1.0

.PHONY: build

run:
	cargo run https://github.com/apinanyogaratnam/opensea-wrapper/tree/master/example

build:
	cargo build --release

tag:
	git tag -m "v${VERSION}" v${VERSION}
	git push --tags

workflow:
	make tag
