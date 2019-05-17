#!/bin/bash

SRC=$PWD/src

mkdir -p target/exe
cd target/exe

g++ $UNICODE -c -o val.o  $SRC/val.cpp
g++ $UNICODE -c -o main.o $SRC/main.cpp
gcc -o main.exe val.o main.o
