#include "statscomponent.hpp"
#include <string>
#include <vector>
#include <iostream>
StatsComponent::StatsComponent() : hp(0), max_hp(0), damage(0) {}
StatsComponent::StatsComponent(int hp, int dmg) : hp(hp), max_hp(0), damage(0) {
  this->hp = hp;
  this->max_hp = hp;
  this->damage = dmg;
}
