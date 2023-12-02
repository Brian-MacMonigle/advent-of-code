run: input test build
	./main.exe

build: main.rs
	rustc main.rs

test: main.rs
	rustc main.rs --test
	./main.exe

input:
	touch input.txt

clean:
	rm -rf main.exe main.pdb