# Allium Starter

This CLI allows you to start quickly your application in local or dev environment

## Installation

```
cargo install --git https://gitlab.com/rsignavong/allium_starter.git --force

```

## Project Setup

First you need to have the following folders structure tree

```
.
+-- bootstrap
|   +-- package.json
|   +-- ...
+-- generated_allium
|   +-- bootstrap
+-- procon
|   +-- mix.exs
|   +-- ...
+-- local.exs
```

## Usage

```
❯ allium-starter -h
allium-starter 0.1.7
Allium Enterprise
Script to speed up allium local development or demo

USAGE:
    allium-starter [OPTIONS]

OPTIONS:
    -a, --register-schemas
            Register schemas in schema registry

    -b, --bootstrap
            Install bootstrap

    -B, --frontend-build
            Build MFs

    -c, --clean
            Clean up generated allium source project (.components.json, backend, frontend)

    -C, --cache
            Purge cache folder including backend and frontend dependencies

        --component <COMPONENT>
            define selected component for other options (e.g. CommandUser.processors.command)

    -d, --database
            Setup database

    -f, --force-generate
            Delete duplicate migrations files and force generation of selected COMPONENT

    -F, --force-generate-and-migrate-compile
            Install MFs dependencies and applications dependencies for selected COMPONENT

    -g, --generate
            Generate only components and config

    -G, --generate-all
            Generate all the allium project and processors and setup local.exs

    -h, --help
            Print help information

    -I, --frontend-install
            Install MFs dependencies and applications dependencies

        --kafka-broker <KAFKA_BROKER>
            Kafka broker [env: KAFKA_BROKER=localhost:9092] [default: localhost:9092]

        --kafka-consumer-groups-cmd <KAFKA_CONSUMER_GROUPS_CMD>
            Kafka consumer groups command path [env: KAFKA_CONSUMER_GROUPS_CMD=]

        --kafka-topic-replication-factor <KAFKA_TOPIC_REPLICATION_FACTOR>
            Kafka topic replication factor [env: KAFKA_TOPIC_REPLICATION_FACTOR=] [default: 3]

        --kafka-topics-cmd <KAFKA_TOPICS_CMD>
            Kafka topics command path [env:
            KAFKA_TOPICS_CMD=/Users/rockysignavong/workspace/confluent-6.1.3/bin/kafka-topics]

    -m, --mix
            Mix up the allium project

    -o, --open <MAESTRO_APPLICATION>
            Open maestro application in browser

    -P, --project-path <PROJECT_PATH>
            Define project folder to run allium-starter [env: PROJECT_PATH=./generated_allium]
            [default: ./generated_allium]

    -s, --server
            Start the server

    -t, --create-topics
            Create topics in kafka

    -V, --version
            Print version information

    -z, --register-materialize-processors
            Register materialize processors (materialize must be started)

        --merge-config <CONFIG_TO_MERGE>
            When generating the project specifies another configuration (a stringified JSON) to merge to the project's configuration

    -e, --environment
            Set the environment to use for PG dump and reading of the `setup.<environment>.yaml` file

```

You can combine multiple options.

```
allium-starter -bcCgmds
```

or

```
allium-starter -f --component CommandEntity -s
```

By default the allium-starter will run on `./generated_allium` project folder, you can define the project folder with `-P, --project <PROJECT_FOLDER>`.

### Options

#### -b, --bootstrap

Run `npm install -g .` inside `bootstrap` folder.

#### -B, --frontend-build

Run `npm build` inside `MFs` folder.

#### -C, --cache

Run `rm -rf cache` inside `generated_allium` folder.

#### -c, --clean

Run `rm -rf .components.json backend frontend` inside `generated_allium` folder.

#### -d, --database

Run `mix ecto.drop && mix ecto.create && mix ecto.migrate` inside `generated_allium` folder.

#### -I, --frontend-install

Run `npm install:mf` inside `MFs` folder, and then run `npm install` inside each maestro application in `frontend` folder and in web components.

#### -g, --generate

Run `allium.gen system bootstrap/bootstrap.config.json -cC` inside `generated_allium` folder.

#### -G, --generate-all

Run `allium.gen system bootstrap/bootstrap.config.json -icCf` inside `generated_allium` folder and then `cp local.exs generated_allium/backend/config/local.exs`.

#### -h, --help

Prints help information

#### -m, --mix

Run `mix deps.get && mix compile` inside `generated_allium` folder.

#### -s, --server

Run `KAFKA=kafka1 iex -S mix phx.server` inside `generated_allium` folder.

#### -f, --force

Run `rm -rf backend/priv/<COMPONENT.component>/<COMPONENT.entity>_pg` and `allium.gen system bootstrap/bootstrap.config.json <COMPONENT> -CF ` inside `generated_allium` folder.

#### -F, --force-and-migrate-compile

Run `rm -rf backend/priv/<COMPONENT.component>/<COMPONENT.entity>_pg` and `allium.gen system bootstrap/bootstrap.config.json <COMPONENT> -CF ` and `mix ecto.[drop/create/migrate] -r <REPO>` (backend) or `npm build` (MF) inside `generated_allium` folder.

#### -o, --open

Run `open https://localhost:<APPLICATION_PORT>/`, with `APPLICATION_PORT` determined from the attribute `_applicationPort` in the `package.json` file of the `MAESTRO_APPLICATION` application.
The option `MAESTRO_APPLICATION` is mandatory and is the name of the maestro application in your `bootstrap.config.json`.

#### -P, --project

Run allium-starter on the specific folder with the [Project Setup](#project-setup).
Default use `./generated_allium` when not defined.

#### -e, --environment

Set the environment to use for PG dump and reading of the `setup.<environment>.yaml` file.
Default to `dev` when not defined.

#### -W, --webcomponents-build

Run `npm run build` inside `webcomponents` folder.

## Contribution

To build locally, you can use the following command

```
cargo build
```

or in release mode

```
cargo build --release
```

To install locally for testing, you can use the following command

```
cargo install --path .
```

To publish the package (locally)

```
cargo release --execute patch --no-publish
```
