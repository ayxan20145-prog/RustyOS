#!/bin/sh

cargo build
mkdir -p iso/boot/grub
cp grub.cfg iso/boot/grub
cp target/i686-unknown-none/debug/RS-DOS iso/boot/rs-dos
grub-mkrescue -o rs-dos.iso iso
