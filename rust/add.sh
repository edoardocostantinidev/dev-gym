#!/bin/bash
cargo new --lib --vcs=none --name $2 ./$1/$2 
code ./$1/$2
