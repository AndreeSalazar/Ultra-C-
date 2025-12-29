#pragma once
#include <string>
#include <vector>
#include <iostream>
#include <memory>
#include <string>
#include <memory>
#include <iostream>
#include <cmath>
#include "entity.hpp"
class Player : public Entity {
public:
  int score;
  int hp;
  int max_hp;
  Player();
  void move(float dx, float dy);
  void heal(int amount);
  void take_damage(int amount);
};
