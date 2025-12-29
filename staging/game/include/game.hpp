#pragma once
#include <string>
#include <vector>
#include <iostream>
#include <memory>
#include "checklist.hpp"
#include "player.hpp"
#include "enemy.hpp"
#include <string>
#include <memory>
#include <iostream>
#include <cmath>
#include "object.hpp"
class Game : public Object {
public:
  bool running;
  int state;
  int width;
  int height;
  int frame_count;
  Checklist checklist;
  Game();
  void clear_screen();
  void log(std::string msg);
  void check_collisions(Player p, Enemy e);
  void draw_ui(Player p);
  void draw_map(Player p, Enemy e);
  bool is_alive(Player p);
  int math_demo();
  void input_update(Player p);
  void run_loop();
};
