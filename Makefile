SRC = $(wildcard src/*.rs src/*/*.rs)

default: hnd

hnd: target/debug/hnd

release: target/release/hnd

target/debug/hnd: $(SRC)
	cargo build

target/release/hnd:
	cargo build --release

clean:
	cargo clean

realclean: clean
	rm -f about.hbs about.toml

test:
	cargo test

cov:
	cargo tarpaulin

watch:
	cargo watch -x test

install: release
	cargo install --path .

doc: $(SRC)
	cargo doc

about: about.hbs about.toml

about.hbs: license
about.toml: license

license:
	cargo about init
	cargo about generate about.hbs > license.html

readdoc:
	cargo doc --open

deb:
	cargo deb

deb-install:
	cargo deb --install
