use super::YamlError;

pub struct Yaml;

impl Yaml {
    pub fn from_json(json: &str) -> Result<String, YamlError> {
        let yaml = serde_json::from_str::<serde_yaml::Value>(json)?;
        Ok(serde_yaml::to_string(&yaml)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_json() {
        let json = "{\"a\":1}";
        let res = Yaml::from_json(json).unwrap();
        assert_eq!(
            res,
            r###"---
a: 1
"###
        );
    }

    #[test]
    fn full_json() {
        let json = r###"
{
  "alliumApi": 1,
  "asyncapi": "2.0.0",
  "components": {
    "schemas": {
      "group": {
        "allOf": [
          { "$ref": "./refs/schemas/group_1.0.0.yaml" },
          { "pGWalCdc": true }
        ]
      }
    }
  },
  "dependencies": null,
  "entities": {
    "enveloppe": "debezium",
    "format": "avro",
    "inputs": null,
    "outputs": {
      "group": {
        "avroKeySchemaVersion": 1,
        "avroValueSchemaVersion": 1,
        "bypassException": true,
        "channel": { "name": "allium-int-commands-groups" },
        "keysMapping": {},
        "masterKey": null,
        "serialization": "avro"
      }
    },
    "schemaRegistry": { "$ref": "#/servers/schemasRegistry" },
    "server": { "$ref": "#/servers/kafkaDefault" },
    "version": ">=1.9.0"
  },
  "errors": null,
  "frontend": null,
  "info": {
    "description": "This Is A Command Processor For Entity Group",
    "title": "Command Group",
    "version": "1.0.0"
  },
  "queries": {
    "insert": "WITH inserted_group AS (INSERT INTO groups (name) VALUES (@{name:attribute}@) RETURNING *) SELECT TRUE AS _success, inserted_group.id, inserted_group.name FROM inserted_group UNION SELECT FALSE, NULL, NULL WHERE NOT EXISTS (SELECT 1 FROM inserted_group)"
  },
  "restApi": {
    "globalScope": "none",
    "private": false,
    "restActions": { "create": "none" },
    "singleton": false
  },
  "servers": {
    "kafkaDefault": { "dev": { "host": "localhost", "port": ["9092"] } },
    "materialize": { "dev": { "host": "localhost", "port": 9092 } },
    "schemasRegistry": {
      "dev": {
        "host": "localhost",
        "port": 8081,
        "scheme": "http",
        "url": "http://localhost:8081"
      }
    }
  }
}
        "###;
        let res = Yaml::from_json(json).unwrap();
        assert_eq!(
            res,
            r###"---
alliumApi: 1
asyncapi: 2.0.0
components:
  schemas:
    group:
      allOf:
        - $ref: "./refs/schemas/group_1.0.0.yaml"
        - pGWalCdc: true
dependencies: ~
entities:
  enveloppe: debezium
  format: avro
  inputs: ~
  outputs:
    group:
      avroKeySchemaVersion: 1
      avroValueSchemaVersion: 1
      bypassException: true
      channel:
        name: allium-int-commands-groups
      keysMapping: {}
      masterKey: ~
      serialization: avro
  schemaRegistry:
    $ref: "#/servers/schemasRegistry"
  server:
    $ref: "#/servers/kafkaDefault"
  version: ">=1.9.0"
errors: ~
frontend: ~
info:
  description: This Is A Command Processor For Entity Group
  title: Command Group
  version: 1.0.0
queries:
  insert: "WITH inserted_group AS (INSERT INTO groups (name) VALUES (@{name:attribute}@) RETURNING *) SELECT TRUE AS _success, inserted_group.id, inserted_group.name FROM inserted_group UNION SELECT FALSE, NULL, NULL WHERE NOT EXISTS (SELECT 1 FROM inserted_group)"
restApi:
  globalScope: none
  private: false
  restActions:
    create: none
  singleton: false
servers:
  kafkaDefault:
    dev:
      host: localhost
      port:
        - "9092"
  materialize:
    dev:
      host: localhost
      port: 9092
  schemasRegistry:
    dev:
      host: localhost
      port: 8081
      scheme: http
      url: "http://localhost:8081"
"###
        );
    }

    #[test]
    fn inline_json() {
        let json = r###"{"alliumApi":1,"asyncapi":"2.0.0","components":{"schemas":{"group":{"allOf":[{"$ref":"./refs/schemas/group_1.0.0.yaml"},{"pGWalCdc":true}]}}},"dependencies":null,"entities":{"enveloppe":"debezium","format":"avro","inputs":null,"outputs":{"group":{"avroKeySchemaVersion":1,"avroValueSchemaVersion":1,"bypassException":true,"channel":{"name":"allium-int-commands-groups"},"keysMapping":{},"masterKey":null,"serialization":"avro"}},"schemaRegistry":{"$ref":"#/servers/schemasRegistry"},"server":{"$ref":"#/servers/kafkaDefault"},"version":">=1.9.0"},"errors":null,"frontend":null,"info":{"description":"This Is A Command Processor For Entity Group","title":"Command Group","version":"1.0.0"},"queries":{"insert":"WITH inserted_group AS (INSERT INTO groups (name) VALUES (@{name:attribute}@) RETURNING *) SELECT TRUE AS _success, inserted_group.id, inserted_group.name FROM inserted_group UNION SELECT FALSE, NULL, NULL WHERE NOT EXISTS (SELECT 1 FROM inserted_group)"},"restApi":{"globalScope":"none","private":false,"restActions":{"create":"none"},"singleton":false},"servers":{"kafkaDefault":{"dev":{"host":"localhost","port":["9092"]}},"materialize":{"dev":{"host":"localhost","port":9092}},"schemasRegistry":{"dev":{"host":"localhost","port":8081,"scheme":"http","url":"http://localhost:8081"}}}}"###;
        let res = Yaml::from_json(json).unwrap();
        assert_eq!(
            res,
            r###"---
alliumApi: 1
asyncapi: 2.0.0
components:
  schemas:
    group:
      allOf:
        - $ref: "./refs/schemas/group_1.0.0.yaml"
        - pGWalCdc: true
dependencies: ~
entities:
  enveloppe: debezium
  format: avro
  inputs: ~
  outputs:
    group:
      avroKeySchemaVersion: 1
      avroValueSchemaVersion: 1
      bypassException: true
      channel:
        name: allium-int-commands-groups
      keysMapping: {}
      masterKey: ~
      serialization: avro
  schemaRegistry:
    $ref: "#/servers/schemasRegistry"
  server:
    $ref: "#/servers/kafkaDefault"
  version: ">=1.9.0"
errors: ~
frontend: ~
info:
  description: This Is A Command Processor For Entity Group
  title: Command Group
  version: 1.0.0
queries:
  insert: "WITH inserted_group AS (INSERT INTO groups (name) VALUES (@{name:attribute}@) RETURNING *) SELECT TRUE AS _success, inserted_group.id, inserted_group.name FROM inserted_group UNION SELECT FALSE, NULL, NULL WHERE NOT EXISTS (SELECT 1 FROM inserted_group)"
restApi:
  globalScope: none
  private: false
  restActions:
    create: none
  singleton: false
servers:
  kafkaDefault:
    dev:
      host: localhost
      port:
        - "9092"
  materialize:
    dev:
      host: localhost
      port: 9092
  schemasRegistry:
    dev:
      host: localhost
      port: 8081
      scheme: http
      url: "http://localhost:8081"
"###
        );
    }
}
