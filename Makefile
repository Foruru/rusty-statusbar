PREFIX = /usr

all:
	cargo build --release

install:
	mkdir -p ${PREFIX}/bin
	cp -f ./target/release/rusty_status_bar ${PREFIX}/bin
