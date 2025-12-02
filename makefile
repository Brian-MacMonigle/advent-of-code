path = ./src/bin/day-$(day)

define newline


endef

# Note, this must be escaped for echo-ing in $(path)/main.rs
define INIT_MAIN
use std::env;
use std::fs;

fn main() {
    println!("Hello Day $(day)!\\n");

    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(&path).expect(format!("Can not read {}", &path).as_str());
}
endef
export INIT_MAIN

run: test build
	main.exe $(path)/input.txt

# Create folder with main.rs if not exists if not exists
$(path)/main.rs: .check-day
ifeq ("$(wildcard $(path)/main.rs)", "")
	mkdir -p $(path)
	touch $(path)/input.txt
	echo -e '$(subst $(newline),\n,${INIT_MAIN})' > $(path)/main.rs
endif

build: .check-day $(path)/main.rs
	rustc -A unused_variables -A dead_code $(path)/main.rs

test: .check-day $(path)/main.rs
	rustc -A unused_variables -A dead_code $(path)/main.rs --test
	main.exe

.check-day:
ifndef day
	$(error day is undefined)
endif

clean:
	rm -rf main.exe main.pdb