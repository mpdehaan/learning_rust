all: main tests

main:
	rustc tasks.rs -o main

go: clean main
	./main

tests: clean
	rustc --test tasks.rs -o test_app
	./test_app

clean:
	-(rm test)
	-(rm main)

