EXEC_NAME = "spreadsheet"
EXEC_DIR = "target/release"
EXTENSION_DIR = "src/extensions"
LATEX_COMP = "pdflatex"
REPORT_DIR = "report"
REPORT_NAME = "report"

$(EXEC_DIR)/$(EXEC_NAME):
	cargo build --release

coverage:
	cargo tarpaulin --exclude-files $(EXTENSION_DIR)/*

test:
	cargo test

docs:
	cargo doc
	$(LATEX_COMP) $(REPORT_DIR)/$(REPORT_NAME).tex
	rm $(REPORT_NAME).aux $(REPORT_NAME).out $(REPORT_NAME).log

ext1:
	cargo run --release 100 100 --extension