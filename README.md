# kafka-admin-cli

A simple Rust command-line tool to manage Apache Kafka via the CLI.

Toy project showcasing Rust's `clap` CLI parser, `tokio` async runtime, and `rdkafka` client.

## Features

✅ Create Kafka topics  
✅ Delete Kafka topics  
✅ List Kafka topics with timeout

## Example Usage

```bash
# Build the binary
cargo build --release

# Create a topic
./kafka-admin-cli --brokers localhost:9092 topics create --name my_topic --partitions 3 --replication 2

# Delete a topic
./kafka-admin-cli --brokers localhost:9092 topics delete --name my_topic

# List topics
./kafka-admin-cli --brokers localhost:9092 topics list --timeout 1s
