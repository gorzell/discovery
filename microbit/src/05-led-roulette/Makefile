
.PHONY: bootstrap
bootstrap:
	#sudo apt-get install gdb-multiarch libusb-1.0-0-dev libusb-dev
	cargo install cargo-binutils --vers 0.3.3
	cargo install cargo-embed --vers 0.11.0

.PHONY: build
build:
	cargo build

.PHONY: flash
flash:
	cargo embed
