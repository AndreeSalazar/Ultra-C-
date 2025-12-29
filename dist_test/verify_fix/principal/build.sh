#!/usr/bin/env bash
set -e
if [ -f "include/pch.hpp" ]; then
  g++ -std=c++17 -x c++-header include/pch.hpp -o build/obj/pch.hpp.gch || clang++ -std=c++17 -x c++-header include/pch.hpp -o build/obj/pch.hpp.gch
fi
g++ -std=c++17 -Wall -Wextra -Werror -fvisibility=hidden -I include -include include/pch.hpp src/*.cpp -o build/bin/principal.exe || clang++ -std=c++17 -Wall -Wextra -Werror -fvisibility=hidden -I include -include include/pch.hpp src/*.cpp -o build/bin/principal.exe
