#!/usr/bin/env node
const util = require('util');
const hello = require('./node-api');

console.log(util.inspect(hello));
console.log(hello.hello())
