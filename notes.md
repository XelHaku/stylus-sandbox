# notes

Arbitrum Sepolia (Testnet)	https://sepolia-rollup.arbitrum.io/rpc	421614	https://sepolia.arbiscan.io	Sepolia	Nitro (Rollup)	wss://sepolia-rollup.arbitrum.io/feed	https://sepolia-rollup-sequencer.arbitrum.io/rpc



cast send 0xD9bF105CD8A3F3A4A3AE57aE9fB1b954a529b955 \
    --value 1000000000000000000 \
    --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
    --rpc-url http://127.0.0.1:8547 \
    --gas-limit 21000 \
    --chain-id 412346


cargo stylus deploy --private-key 0x9dc8c652a16755e58fe2ffae9991c895174f3746022b2a474f8d89694b4dc312 -e https://sepolia-rollup.arbitrum.io/rpc



cargo stylus cache bid <ADDRESS> 0 --private-key <PRIVATE_KEY> -e <RPC_URL>



docker stop nitro-dev
docker restart


test



chmod 644 /home/polar/git/stylus-sandbox/rust-toolchain.toml
