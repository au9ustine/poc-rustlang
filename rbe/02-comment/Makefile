compile: doc.rs
	rustc $< --crate-type=lib

generate-doc: doc.rs
	rustdoc --test --extern doc="libdoc.rlib" doc.rs
