all: tests
	rustc fizzbuzz.rs -o main
tests:
	rustc --test fizzbuzz.rs -o tests
	./tests

clean:
	-(rm test)
	-(rm main)

