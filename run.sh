#!/bin/bash
#test branches

cp /autograder/submission/Cargo.lock ./
cp /autograder/submission/Cargo.toml ./
cp -r /autograder/submission/src ./

/root/.cargo/bin/cargo run
