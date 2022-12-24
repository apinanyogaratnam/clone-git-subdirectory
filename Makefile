run:
	cargo run https://github.com/vercel/next.js/tree/deprecated-main/examples/active-class-name

tag:
	git tag -m "v${VERSION}" v${VERSION}
	git push --tags

workflow:
	make tag
