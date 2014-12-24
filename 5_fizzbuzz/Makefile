all: tests
	rustc fizzbuzz.rs -o main
tests:
	rustc --test fizzbuzz.rs -o test_app
	./test_app

clean:
	-(rm test)
	-(rm main)

