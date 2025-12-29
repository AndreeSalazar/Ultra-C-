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
class Enemy : public Entity {
public:
  int damage;
  Enemy();
  Enemy(float x, float y);
};
