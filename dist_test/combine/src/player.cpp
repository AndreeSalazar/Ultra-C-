#include "pch.hpp"
#include "player.hpp"
#include "rect.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
Player::Player() : name(std::string()) {}
Player::Player(std::string name) : name(name) {
}
void Player::render() UCPP_NOEXCEPT {
  std::cout << std::string("Player: ") + name << std::endl;
}
Rect Player::get_rect() const UCPP_NOEXCEPT {
  Rect r; r.x = x; r.y = y; r.w = 1.0f; r.h = 1.0f; return r;
}
}
