#pragma once
#include <string>
#include <vector>
#include <iostream>
#include <memory>
#include <map>
#include <list>
#include <optional>
#include <thread>
#include <mutex>
#include <future>
#include <atomic>
#include <filesystem>
#include <fstream>
#include <algorithm>
#include <numeric>
#include <cmath>
#include <cstdio>
#include <functional>
namespace Juego {
class Strings {
public:
  std::map<std::string, std::string> table;
  Strings();
  Strings(std::map<std::string, std::string> table);
  void load(std::string lang);
  std::string get(std::string key) const;
};
}
