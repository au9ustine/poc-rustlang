compile: print.rs
	rustc $<

.PHONY:
test: compile
	./print

clean: print
	rm -vf $<
