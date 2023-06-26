#!/bin/sh

cd ./src/

make clean
TARGET=release make

cd ../
