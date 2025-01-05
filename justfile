#!/usr/bin/env -S just --justfile

alias b := build
alias r := run

flags := '--color=always'

# print infor
default:
    just -l

# builds the dev executable and checks quality standards
build:
    cargo mommy build {{flags}} && cargo mommy clippy {{flags}}

# run the application
run:
    cargo mommy run {{flags}}

# run all tests present
test-all:
    cargo mommy test {{flags}}

test TEST:
    cargo mommy test --test {{TEST}} {{flags}}

format:
    cargo fmt --all

