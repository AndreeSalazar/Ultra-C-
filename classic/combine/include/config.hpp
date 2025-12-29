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
#include "rect.hpp"
namespace Juego {
class Config {
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
  static Config load(std::string path);
};
}
