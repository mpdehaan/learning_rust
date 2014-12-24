all: main tests

main:
	rustc structs.rs -o main

go: clean main
	./main

tests: clean
	rustc --test structs.rs -o test_app
	./test_app

clean:
	-(rm test)
	-(rm main)

