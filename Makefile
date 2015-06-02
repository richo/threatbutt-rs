all: sample attribute

sample: examples/sample.rs lib
	rustc -L target/debug -L target/debug/deps $< -o $@

attribute: examples/attribute.rs lib
	rustc -L target/debug -L target/debug/deps $< -o $@


lib:
	cargo build
.PHONY: lib

