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
	dfx canister create backend
	dfx canister create bounty
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install backend --mode reinstall --yes
	dfx canister install bounty --mode reinstall --yes

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install backend --mode=upgrade
	dfx canister install bounty --mode=upgrade

.PHONY: test
.SILENT: test
test-a: install
	# Call the bounty canister to get the GitHub issue and capture the output
	@echo "Calling healthcheck on bounty canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call bounty healthcheck > $$TMP_FILE; \
	echo "healthcheck response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test
.SILENT: test
test-1: install
	# Call the backend canister to get the GitHub issue and capture the output
	@echo "Calling get_gh_issue on backend canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call backend get_gh_issue '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_gh_issue response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test
.SILENT: test
test-2: install
	# Call the backend canister to get the GitHub PR that close some issue and capture the output
	@echo "Calling get_gh_fixed_by on backend canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call backend get_gh_fixed_by '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_gh_fixed_by response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: test
.SILENT: test
test-3: install
	# Call the backend canister to get the GitHub PR merge status and capture the output
	@echo "Calling get_gh_is_merged on backend canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call backend get_gh_is_merged '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_gh_is_merged response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE	

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx