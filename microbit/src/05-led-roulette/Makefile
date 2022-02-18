
.PHONY: bootstrap
bootstrap:
	cargo install cargo-binutils --vers 0.3.3
	cargo install cargo-embed --vers 0.11.0
	#sudo apt-get install gdb-multiarch libusb-1.0-0-dev libusb-dev

.PHONY: build
build:
	cargo build --features v2 --target thumbv7em-none-eabihf

.PHONY: flash
flash:
	cargo embed --features v2 --target thumbv7em-none-eabihf
