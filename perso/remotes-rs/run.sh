#!/bin/sh

wasm-pack build app --target web --out-dir ../tmp/static/app/ 

mkdir -p tmp/static/css/
npm run --prefix styles build
cp styles/dist/styles.css tmp/static/css/styles.css

cargo run 
