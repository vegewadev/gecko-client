# Gecko Analytics

This Rust project is designed to run on a Raspberry Pi 4 and collect analytics data from a DHT11 module connected to the device. The collected data is then written to a MongoDB database for storage and analysis.

## Prerequisites

Before running this project, ensure that you have the following prerequisites installed:

- Rust programming language (latest stable version)
- MongoDB (either locally or a remote instance)

## Installation

1. Clone the repository:

```

git clone https://github.com/your-username/gecko-analytics.git

```

2. Change to the project directory:

```

cd gecko-analytics

```

3. Build the project:

```

cargo build --release

```

## Running the Project

### On Raspberry Pi 4

1. Connect the DHT11 module to the Raspberry Pi 4 according to the wiring instructions.

2. Run the compiled binary:

```

./target/release/gecko-analytics

```

### Using Docker (arm64 only)

You can also run this project using Docker by pulling the pre-built image:

```

docker pull ghcr.io/vegewadev/gecko-client:latest

```

Then, run the container with the necessary environment variables:

```

docker run --platform linux/arm64 --privileged -v /dev/gpiomem:/dev/gpiomem -e CONNECTION_STRING="mongodb://mongo:27017/" -e RUST_BACKTRACE=1 ghcr.io/vegewadev/gecko-client:latest


```

Replace `CONNECTION_STRING` with the appropriate connection string for your MongoDB instance.

## Configuration

The project can be configured using environment variables:

- `CONNECTION_STRING`: The connection string for the MongoDB instance (required).

## License

This project is licensed under the [MIT License](LICENSE).
