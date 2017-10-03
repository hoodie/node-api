#!/bin/bash

curl -O "https://raw.githubusercontent.com/nodejs/node/master/src/node_api.h"
curl -O "https://raw.githubusercontent.com/nodejs/node/master/src/node_api_types.h"

bindgen node_api.h > src/lib.rs
