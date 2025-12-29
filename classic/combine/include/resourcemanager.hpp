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
class ResourceManager {
public:
  std::map<std::string, std::string> resources;
  std::map<std::string, int> refs;
  ResourceManager();
  ResourceManager(std::map<std::string, std::string> resources, std::map<std::string, int> refs);
  void load(std::string name, std::string content);
  std::string get(std::string name) const;
  void release(std::string name);
};
}
