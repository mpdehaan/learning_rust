all: main tests

main:
	rustc fizzbuzz.rs -o main

go: main
	./main

tests:
	rustc --test fizzbuzz.rs -o test_app
	./test_app

clean:
	-(rm test)
	-(rm main)

