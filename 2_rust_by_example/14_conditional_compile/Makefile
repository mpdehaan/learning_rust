all:
	rustc main.rs --cfg some_condition=bar -o main1
	rustc main.rs --cfg some_condition=foo -o main2
	./main1
	./main2
