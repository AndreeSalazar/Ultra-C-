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
#include "player.hpp"
#include "rect.hpp"
#include "eventbus.hpp"
#include "resourcemanager.hpp"
#include "strings.hpp"
namespace Juego {
class UCPP_API Principal {
public:
  bool running;
  int score;
  int level;
  bool paused;
  int target_fps;
  int width;
  int height;
  int high_score;
  std::string profile;
  Player player;
  std::vector<Rect> obstacles;
  EventBus bus;
  ResourceManager rm;
  std::string ascii_player;
  std::string ascii_obstacle;
  Strings strings;
  Principal();
  Principal(bool running, int score, int level, bool paused, int target_fps, int width, int height, int high_score, std::string profile, Player player, std::vector<Rect> obstacles, EventBus bus, ResourceManager rm, std::string ascii_player, std::string ascii_obstacle, Strings strings);
  void update(float dt);
  void draw();
  void run_loop();
  void hola_upp();
  void start();
  void loop();
  void run();
};
}
