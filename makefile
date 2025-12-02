path = ./src/bin/day-$(day)

run: .check-day input test build
	main.exe $(path)/input.txt

build: .check-day $(path)/main.rs
	rustc $(path)/main.rs

test: .check-day $(path)/main.rs
	rustc $(path)/main.rs --test
	main.exe

input: .check-day
	touch $(path)/input.txt

.check-day:
ifndef day
	$(error day is undefined)
endif

clean:
	rm -rf main.exe main.pdb