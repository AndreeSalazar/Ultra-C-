#!/usr/bin/env bash
set -e
g++ -std=c++17 -O2 -I include src/all.cpp -o build/bin/06_emision_modos.exe || clang++ -std=c++17 -O2 -I include src/all.cpp -o build/bin/06_emision_modos.exe
