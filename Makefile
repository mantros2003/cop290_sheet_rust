EXEC_DIR = "target/release"
EXEC_NAME = "sheet"

$(EXEC_DIR)/$(EXEC_NAME):
	cargo build --release

test: $(EXEC_DIR)/$(EXEC_NAME)
	cargo run --release