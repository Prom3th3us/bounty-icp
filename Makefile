SHELL = /bin/bash

# Load environment variables from .env file
include .env

.PHONY: all
all: build

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	npm install

.PHONY: build
.SILENT: build
build: # node_modules
	dfx canister create \
		--with-cycles 1_000_000_000_000 \
		--specified-id rdmx6-jaaaa-aaaaa-aaadq-cai \
		identity
	dfx canister create \
		--with-cycles 1_000_000_000_000 \
		--specified-id mxzaz-hqaaa-aaaar-qaada-cai \
		icrc1_ledger
	dfx canister create \
		--with-cycles 1_000_000_000_000 \
		--specified-id n5wcd-faaaa-aaaar-qaaea-cai \
		icrc1_index
	dfx canister create backend
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install identity --argument '(null)' --mode reinstall --yes
	./make/install_ledger.sh
	./make/install_ledger_index.sh
	./make/install_backend.sh

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install identity --argument '(null)' --mode=upgrade
	dfx canister install icrc1_ledger --mode=upgrade
	dfx canister install icrc1_index --argument '(null)' --mode=upgrade
	dfx canister install backend --mode=upgrade

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr node_modules
	rm -fr target

# tests
.PHONY: test-1
.SILENT: test-1
test-1: # install
	# Call the backend canister to get the GitHub issue and capture the output
	@echo "Calling get_issue on backend canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call backend get_issue '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_issue response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test-2
.SILENT: test-2
test-2: # install
	# Call the backend canister to get the GitHub PR that close some issue and capture the output
	@echo "Calling get_fixed_by on backend canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call backend get_fixed_by '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_fixed_by response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test-3
.SILENT: test-3
test-3: # install
	# Call the backend canister to get the GitHub PR merge status and capture the output
	@echo "Calling get_is_merged on backend canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call backend get_is_merged '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_is_merged response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test-4
.SILENT: test-4
test-4: # install
	# Call the backend canister to get the GitHub closing PR details and capture the output
	@echo "Calling get_merged_details on backend canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call backend get_merged_details '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_merged_details response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE
	

.PHONY: test-5
.SILENT: test-5
test-5: # install
	# Call the backend canister to find if the user is registered or not on GitHub and capture the output
	@echo "Calling get_user_exists on backend canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call backend get_user_exists '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_user_exists response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test
.SILENT: test
test: install
	# Call the backend canister for healthcheck and capture the output
	@echo "Calling healthcheck on backend canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call backend healthcheck > $$TMP_FILE; \
	echo "healthcheck response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test-deposit
.SILENT: test-deposit
test-deposit: # install
	./make/test/deposit.sh
