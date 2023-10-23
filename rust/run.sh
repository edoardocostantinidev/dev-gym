#!/bin/bash
for dir in ./$1/*; do (cd $dir;cargo test); done
