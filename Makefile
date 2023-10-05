BIN_NAME = revshell
INSTALL_DIR = /usr/local/bin

install:
	cargo build --release
	mkdir -p $(INSTALL_DIR)
	cp target/release/$(BIN_NAME) $(INSTALL_DIR)
	@echo "Installed $(BIN_NAME) to $(INSTALL_DIR)"

uninstall:
	rm -f $(INSTALL_DIR)/$(BIN_NAME)
	@echo "Uninstalled $(BIN_NAME) from $(INSTALL_DIR)"

.PHONY: install uninstall

