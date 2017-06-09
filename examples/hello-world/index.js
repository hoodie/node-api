#!/usr/bin/env node
const util = require('util');
const hello = require('./node-api');

console.log("javascript:", util.inspect(hello));
console.log("javascript:", hello.hello())
console.log("javascript:", hello.add(2))
