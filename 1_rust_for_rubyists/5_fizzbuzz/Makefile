all: main tests

main:
	rustc fizzbuzz.rs -o main

go: clean main
	./main

tests: clean
	rustc --test fizzbuzz.rs -o test_app
	./test_app

clean:
	-(rm test)
	-(rm main)

