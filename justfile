doc:
	RUSTDOCFLAGS="--cfg docsrs" cargo doc --all-features --no-deps

opendoc:
	cargo doc --open --no-deps
