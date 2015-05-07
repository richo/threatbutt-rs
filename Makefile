all: sample attribute

sample: example/sample.rs lib
	rustc -L target/debug -L target/debug/deps $< -o $@

attribute: example/attribute.rs lib
	rustc -L target/debug -L target/debug/deps $< -o $@


lib:
	cargo build
.PHONY: lib

