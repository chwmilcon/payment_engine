build:
    cargo build

test:
    cargo test

itest:
    cargo test --test test_transaction_processing

runsample1:
    cargo run -- -d --logfile=log.txt sample_data/sample1.txt

runsample2:
    cargo run -- -d --logfile=log.txt sample_data/err3.txt
