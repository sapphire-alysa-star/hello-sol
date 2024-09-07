
building the program:
cargo build-sbf
solana program deploy ./target/deploy/hello_world.so

node script:
node client.mjs


test-validator:
cd ~
solana-test-validator