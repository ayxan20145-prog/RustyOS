#!/bin/sh

cargo build
mkdir -p iso/boot/grub
cp grub.cfg iso/boot/grub
cp target/i686-unknown-none/debug/RustyOS iso/boot/rustyos
grub-mkrescue -o rustyos.iso iso
