#!/bin/bash

BINPATH=target/x86_64-unknown-linux-musl/release/remrs
[[ "$NOSUDO" != y ]] && SUDOCMD=sudo


case "$1" in
	b | build )
		RUSTC_BOOTSTRAP=1 RUSTFLAGS="-C link-self-contained=no" cargo zigbuild --release   --target x86_64-unknown-linux-musl   -Z build-std=std,panic_abort
		;;
	ii | install_local )
		install -vm755 "$BINPATH" "$HOME"/.local/bin/remrs
		;;
	i | install )
		$SUDOCMD install -vm755 "$BINPATH" /usr/local/bin/remrs
		;;
	all | '' )
		./make.sh build &&
		./make.sh ii &&
		./make.sh i
		;;
esac
