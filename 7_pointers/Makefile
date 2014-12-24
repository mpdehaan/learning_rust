all: main tests

main:
	rustc pointers.rs -o main

go: clean main
	./main

tests: clean
	rustc --test pointers.rs -o test_app
	./test_app

clean:
	-(rm test)
	-(rm main)

