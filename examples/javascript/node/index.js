const jels = require('jellyschema');

const initialValue = `
title: demo
version: 1
properties:
  - network:
      title: Network
      properties:
        - ssid:
            title: Network SSID
            type: string
            minLength: 1
            maxLength: 32
        - passphrase:
            title: Network Key
            type: password
            minLength: 8
`;

var schema = new jels.JellySchema(initialValue);
const valid = schema.validate({ network: { ssid: "FOO", passphrase: "BAR" }});
console.log("Schema is valid:", valid);
console.log("Errors:", schema.errors());
