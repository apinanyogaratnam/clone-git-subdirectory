VERSION := 0.1.0

.PHONY: build

run:
	cargo run https://github.com/vercel/next.js/tree/deprecated-main/examples/active-class-name

build:
	cargo build --release

tag:
	git tag -m "v${VERSION}" v${VERSION}
	git push --tags

workflow:
	make tag
