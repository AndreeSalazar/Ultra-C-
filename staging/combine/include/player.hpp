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
#include "rect.hpp"
#include "entity.hpp"
namespace Juego {
class UCPP_API Player : public Entity {
public:
  std::string name;
  Player();
  Player(std::string name);
  void render() UCPP_NOEXCEPT;
  Rect get_rect() const UCPP_NOEXCEPT;
};
}
