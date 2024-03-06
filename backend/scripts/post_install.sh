#!/bin/bash
echo post_install task: candid-extractor
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm >./backend/canisters/backend/backend.did