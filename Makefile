ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 11681299
STOP_BLOCK ?= +500

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_vault_share_transfers -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package: build
	substreams package substreams.yaml
