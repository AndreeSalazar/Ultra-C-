#pragma once
#include <string>
#include <vector>
#include <iostream>
#include <memory>
#include <string>
#include <memory>
#include <iostream>
#include <cmath>
#include "object.hpp"
class StatsComponent : public Object {
public:
  int hp;
  int max_hp;
  int damage;
  StatsComponent();
  StatsComponent(int hp, int dmg);
};
