prod_build:
	RUSTFLAGS='-C target-cpu=native' cargo build --release --features jemalloc
new_no_code:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- new --path $(CURDIR)/../examples no-code
new_rust:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- new --path $(CURDIR)/../../rust rust
start_indexer:
	RUSTFLAGS='-C target-cpu=native' RUST_BACKTRACE='full' cargo run --release --features jemalloc -- start --path $(CURDIR)/../examples/rindexer_demo_cli indexer
start_all:
	RUSTFLAGS='-C target-cpu=native' RUST_BACKTRACE='full' cargo run --release --features jemalloc -- start --path $(CURDIR)/../examples/rindexer_demo_cli all
start_graphql:
	RUSTFLAGS='-C target-cpu=native' RUST_BACKTRACE='full' cargo run --release --features jemalloc -- start --path $(CURDIR)/../examples/rindexer_demo_cli graphql
codegen_typings:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- codegen --path $(CURDIR)/../rindexer_rust_playground typings
codegen_indexer:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- codegen --path $(CURDIR)/../rindexer_rust_playground indexer
codegen_graphql:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- codegen --path $(CURDIR)/../examples/rindexer_demo_cli graphql --endpoint  http://0.0.0.0:5005/graphql
add_contract:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- add --path $(CURDIR)/../examples/rindexer_demo_cli contract
delete:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- delete --path $(CURDIR)/../examples/rindexer_demo_cli
phantom_init:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- phantom --path $(CURDIR)/../examples/rindexer_demo_cli init
phantom_clone:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- phantom --path $(CURDIR)/../examples/rindexer_demo_cli clone --contract-name RocketPoolETH --network ethereum
phantom_compile:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- phantom --path $(CURDIR)/../examples/rindexer_demo_cli compile --contract-name RocketPoolETH --network ethereum
phantom_deploy:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- phantom --path $(CURDIR)/../examples/rindexer_demo_cli deploy --contract-name RocketPoolETH --network ethereum
help:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- phantom --help

################################################################################
# LOCAL NONE CHECKED IN PROJECT COMMANDS
################################################################################
start_indexer_base_paint:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- start --path $(CURDIR)/../examples/base_paint indexer
start_graphql_base_paint:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- start --path $(CURDIR)/../examples/base_paint graphql
start_indexer_lens_mirrors:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- start --path $(CURDIR)/../examples/lens_mirrors indexer
start_uniswap:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- start --path $(CURDIR)/../examples/uniswap_v3_factory all
start_sophon:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- start --path $(CURDIR)/../examples/sophon_testnet_contract_deploy all
start_indexer_issue:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- start --path $(CURDIR)/../../rindexer-issue/smallIndexer indexer
codegen_typings_lens:
	cargo run -- codegen --path $(CURDIR)/../../lens-backend/crates/lens-indexer typings
codegen_indexer_lens:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- codegen --path $(CURDIR)/../../lens-backend/crates/indexer indexer
start_uniswap_base:
	RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- start --path $(CURDIR)/../examples/uniswap_v3_base indexer
start_indexer_redis_stream:
    RUSTFLAGS='-C target-cpu=native' cargo run --release --features jemalloc -- start --path $(CURDIR)/../examples/redis-stream indexer
