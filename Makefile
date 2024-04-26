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
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install backend --mode reinstall --yes

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install backend --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	# Call the backend canister to get the GitHub issue and capture the output
	@echo "Calling get_gh_issue on backend canister..."
	@TMP_FILE=$$(mktemp); \
	dfx canister call backend get_gh_issue '("${GITHUB_TOKEN}")' > $$TMP_FILE; \
	echo "get_gh_issue response:"; \
	cat $$TMP_FILE; \
	rm -f $$TMP_FILE

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx