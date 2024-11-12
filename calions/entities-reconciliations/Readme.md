# Entities Reconciliations Processor

## Requirements

- Rust
- Cargo
- Kafka

## Required Kafka Topics

- calions-int-evt-entities_{entity}_{entity}_reconciliations_data
- calions-int-evt-entities_{entity}_{entity}_reconciliations
- calions-int-evt-entities_{entity}_{entity}_reconciliations_status

## Development

### Setup

You will need to install Rust on your local machine, follow the instructions on [Rust Official website](https://www.rust-lang.org/tools/install).

The installation will come with 2 binaries: `rustc` and `cargo`.
Rustc is the rust compiler like gcc but for Rust.
Cargo is the Rust project and package maanager called [Crates](https://crates.io/) in Rust terminologies.
You will only use `cargo` as it wraps all the `rustc` commands.

## Check, Build and Run

### Check

On development mode, you can check the application sources code Syntax, Typings and Borrowchecking with `check`. It is very fast compilation.

```
$ cargo check
```

### Build

Build is the same as `check` but it will compile sources and build an executable binary file. It is generally used for testing the application without testing performance.
Once build the development binary file can be found in `./target/debug/entities-reconciliations`.

```
$ cargo build
```

For production usage, you need to build in `release mode`. The application will have better performance and the binary size will be smaller, but the compilation is very slow.
Once build the release binary file can be found in `./target/release/entities-reconciliations`.

```
$ cargo build --release
```

You change change the target directory by adding `--target-dir DIRECTORY`.
Please refer to [Cargo book](https://doc.rust-lang.org/cargo/commands/cargo-build.html) for more informations on build.

### Run

To run in development mode, you can use `build first and then run the binary from target directory` or you can run simply:

```
$ cargo run
```

It will compile, build and run automatically the application.

You can do the same in `release mode`.

```
$ cargo run --release
```

## Build and Run on development

You need to install an external cargo tool called [Cargo Watch](https://github.com/passcod/cargo-watch)

```
$ cargo watch -x [check|build|run]
```

## CI Setup

### Build

You need to use a Rust Docker Base Image to compile Rust sources code to an executable binary file.
You can use this [image](https://hub.docker.com/_/rust) from DockerHub.
Then you build the binary using

```
$ cargo run --release
```

and you will find the binary at `./target/release/entities-reconciliations`.

You can specify the target directory

```
$ cargo run --release --target-dir DIRECTORY
```

### Run

Once you get your binary file on the specific target, you may need to move/copy the binary to a new docker image.
As it is an executable binary file, you only need an OS base image (alpine, ubuntu, ...).

### Volumes

The processor needs to create, write and read files and directory on `./data` from the root directory path you are running the binary.
You may add some rights to the `./data` directoy.

To have a better resilience, you may need to link the volumes to an object storage to make a backup.

## Logging

To run the processor with specific log level, you need add `RUST_LOG` environment variable.

```
$ RUST_LOG=warn ./entities-reconciliations (or cargo run or cargo run --release)
```

### Log level

There are 4 kind of logs you can choose:

- info
- warn
- error
- debug

If `RUST_LOG` is not defined, all logs will be displayed.
So you won't have to define the `RUST_LOG` variable for production.

## Topics configuration

### Development

You can change the kafka and topics configuration in the `config/` folder.
It supports the `12-factor` applications.
The `default.toml` file defines all the configurable settings.

You can override the settings by simply overriding the same settings inside your own `config/local.toml` file (gitignored).

Or you can use the .env file with environment variables as below.

### Other Environments

For running app on other environment, you must use the following environment variables.

```
RUST_LOG=entities-reconciliations
APP__KAFKA__CONFIG__BROKERS="localhost:9092,localhost:9093,localhost:9094"
APP__KAFKA__CONFIG__GROUP_ID="calions-entities-reconciliations-workers"
APP__KAFKA__CONFIG__TIMEOUT=5000
APP__KAFKA__CONSUMERS__RECONCILIATION_DATA__TOPICS="calions-int-evt-entities_{entity}_{entity}_reconciliations_data,"
APP__KAFKA__CONSUMERS__RECONCILIATION_COMMAND__TOPICS="calions-int-evt-entities_{entity}_{entity}_reconciliations,"
APP__KAFKA__PRODUCERS__STATUS__TOPIC="calions-int-evt-entities_{entity}_{entity}_reconciliations_statuses"
APP__ORGANIZATION_ID="123"
APP__PROCESSOR_ID="123"
APP__STATUS_HEARTBEAT_INTERVAL=30000
```
