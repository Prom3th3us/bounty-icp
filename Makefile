SHELL := /bin/bash

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
build: node_modules
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
	dfx canister create github
	dfx canister create bounty
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install identity --argument '(null)' --mode reinstall --yes
	$(shell make/install_ledger.sh)
	$(shell make/install_ledger_index.sh)
	dfx canister install github --mode reinstall --yes
	$(shell make/install_bounty.sh)

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install identity --argument '(null)' --mode=upgrade
	dfx canister install icrc1_ledger --mode=upgrade
	dfx canister install icrc1_index --argument '(null)' --mode=upgrade
	dfx canister install github --mode=upgrade
	dfx canister install bounty --mode=upgrade

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr node_modules
	rm -fr target

# tests
.PHONY: test-1
.SILENT: test-1
test-1: install
	# Call the github canister to get the GitHub issue and capture the output
	@echo "Calling get_issue on github canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call github get_issue '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_issue response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test-2
.SILENT: test-2
test-2: install
	# Call the github canister to get the GitHub PR that close some issue and capture the output
	@echo "Calling get_fixed_by on github canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call github get_fixed_by '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_fixed_by response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test-3
.SILENT: test-3
test-3: install
	# Call the github canister to get the GitHub PR merge status and capture the output
	@echo "Calling get_is_merged on github canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call github get_is_merged '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_is_merged response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test-4
.SILENT: test-4
test-4: install
	# Call the github canister to get the GitHub closing PR details and capture the output
	@echo "Calling get_merged_details on github canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call github get_merged_details '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_merged_details response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test-a
.SILENT: test-a
test-a: install
	# Call the bounty canister for healthcheck and capture the output
	@echo "Calling healthcheck on bounty canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call bounty healthcheck > $$TMP_FILE; \
	echo "healthcheck response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test-deposit
.SILENT: test-deposit
test-deposit: install
	$(shell make/test/deposit.sh)
