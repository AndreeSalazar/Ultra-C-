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
namespace Juego {
class UCPP_API Config {
public:
  int width;
  int height;
  int start_x;
  int start_y;
  std::vector<Rect> obstacles;
  std::string profile;
  std::string sprite_player;
  std::string sprite_obstacle;
  std::string audio_map;
  std::string lang;
  Config();
  Config(int width, int height, int start_x, int start_y, std::vector<Rect> obstacles, std::string profile, std::string sprite_player, std::string sprite_obstacle, std::string audio_map, std::string lang);
  void add_obstacle(int x, int y);
  void set_size(int w, int h);
  void set_start(int x, int y);
  static Config load(const std::string& path);
};
}
