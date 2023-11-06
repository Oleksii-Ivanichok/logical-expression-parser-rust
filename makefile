# Environment variables
RUSTC := rustc
CARGO := cargo
PROGRAM_NAME := my_program
TESTS_DIR := tests

# Default target
all: build

# Clean build artifacts
clean:
    $(CARGO) clean

# Install dependencies
install:
    $(CARGO) build

# Build the project
build:
    $(CARGO) build

# Run the program
run:
    $(CARGO) run

# Run tests
test:
    $(CARGO) test

# Package the project
package:
    $(CARGO) build --release

# Deploy the project
deploy:
    # Add deployment commands here

# Lint the code
lint:
    # Add linting commands here

# Check code style
checkstyle:
    # Add checkstyle commands here

# Backup the project
backup:
    # Add backup commands here

# Initialize the project
init:
    # Add initialization commands here