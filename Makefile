main="./main"

compile:
	rustc ./main.rs

run: $(main)
	$<

$(main): compile

