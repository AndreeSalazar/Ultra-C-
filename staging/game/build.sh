#!/usr/bin/env bash
set -e
g++ -std=c++17 -I include src/*.cpp -o build/bin/game.exe || clang++ -std=c++17 -I include src/*.cpp -o build/bin/game.exe
