PREFIX = /usr

all:
	cargo build --release

install: all
	mkdir -p ${PREFIX}/bin
	cp ./target/release/rusty_status_bar ${PREFIX}/bin
