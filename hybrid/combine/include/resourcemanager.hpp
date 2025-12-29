#pragma once
#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
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
#include "pch.hpp"
namespace Juego {
class UCPP_API ResourceManager {
public:
  std::map<std::string, std::string> resources;
  std::map<std::string, int> refs;
  ResourceManager();
  ResourceManager(std::map<std::string, std::string> resources, std::map<std::string, int> refs);
  void load(const std::string& name, const std::string& content);
  std::string get(const std::string& name) const UCPP_NOEXCEPT;
  void release(const std::string& name) UCPP_NOEXCEPT;
};
}
