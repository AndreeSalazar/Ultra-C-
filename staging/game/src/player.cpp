#include "player.hpp"
#include <string>
#include <vector>
#include <iostream>
Player::Player() : score(0), hp(0), max_hp(0) {
  
  this->x = 1.0;
  this->y = 1.0;
  this->symbol = "@";
  this->score = 0;
  this->hp = 100;
  this->max_hp = 100;
  this->active = true;
}
void Player::move(float dx, float dy) {
  this->x += dx; this->y += dy;
}
void Player::heal(int amount) {
  this->hp += amount; if(this->hp > this->max_hp) this->hp = this->max_hp;
}
void Player::take_damage(int amount) {
  this->hp -= amount; if(this->hp < 0) this->hp = 0;
}
