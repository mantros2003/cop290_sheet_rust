EXEC_NAME = "spreadsheet"
EXEC_DIR = "target/release"
EXTENSION_DIR = "src/extensions"

$(EXEC_DIR)/$(EXEC_NAME):
	cargo buil --release