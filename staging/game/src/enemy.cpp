#include "enemy.hpp"
#include <string>
#include <vector>
#include <iostream>
Enemy::Enemy() : damage(0) {}
Enemy::Enemy(float x, float y) : damage(0) {
  
  this->x = x;
  this->y = y;
  this->symbol = "E";
  this->active = true;
  this->damage = 10;
}
