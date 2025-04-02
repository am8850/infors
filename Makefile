default:
	@echo "Please run 'make help' for a list of available commands."
	@echo "Or run 'make <command>' to execute a specific command."

build:
	@echo "Building the project..."
	cargo build
	@echo "Build completed."
	
build-release:
	@echo "Building the project..."
	cargo build --release
	@echo "Build completed."	

install:
	cargo install --path .
