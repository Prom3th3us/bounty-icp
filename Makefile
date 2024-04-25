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
	dfx canister call backend get_icp_usd_exchange \
		| grep '\[1682978460,5\.714,5\.718,5\.714,5\.714,243\.5678\]' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx