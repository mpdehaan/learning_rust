all: main tests

main:
	rustc vectors.rs -o main

go: clean main
	./main

tests: clean
	rustc --test vectors.rs -o test_app
	./test_app

clean:
	-(rm test)
	-(rm main)

