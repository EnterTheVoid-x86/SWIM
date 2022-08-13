prog :=swim

debug ?=


ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

build:
	cargo build $(release)

install:
	sudo cp target/$(target)/$(prog) /usr/bin/$(prog)
	sudo cp swimkeybinds /usr/bin/swimkeybinds
	sudo cp keybinds /usr/bin/keybinds
	sudo cp SWIM.desktop /usr/share/xsessions/

all: build install
 
help:
	@echo "Make sure you have Picom, ST, dmenu-run, feh, and rust installed to use the default configuration for SWIM."
	@echo "usage: make [debug=1]"
