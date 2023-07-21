build:
	@cargo build
	./target/debug/betterll
	@rm -f ./betterll
	@ln -s ./target/debug/betterll ./betterll
