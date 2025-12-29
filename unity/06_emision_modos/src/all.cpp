#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include <algorithm>
#include <map>
#include <list>
#include <optional>
#include <thread>
#include <mutex>
#include <future>
#include <atomic>
#include <filesystem>
#include <fstream>
#include <numeric>
#include <cmath>
#include <cstdio>
#include <functional>

// Forward Declarations
class Principal;

// Class Definitions
class Principal {
public:
  void run();
};


// Class Implementations
void Principal::run() {
  std::cout << "Modo de emisión listo" << std::endl;
}


#ifdef _WIN32
#include <windows.h>
#endif

int main() {
  #ifdef _WIN32
    SetConsoleOutputCP(65001);
  #endif
  try {
    Principal app;
    app.run();
  } catch (const std::exception& e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
  return 0;
}
