#!/bin/bash

cp /autograder/submission/Cargo.lock ./
cp /autograder/submission/Cargo.toml ./
cp /autograder/submission/src ./

/root/.cargo/bin/cargo run
