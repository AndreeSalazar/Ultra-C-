#include "config.hpp"
#include "pch.hpp"
#include "sound.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
Config::Config() : width(0), height(0), start_x(0), start_y(0), obstacles(std::vector<Rect>()), profile(std::string()), sprite_player(std::string()), sprite_obstacle(std::string()), audio_map(std::string()), lang(std::string()) {}
Config::Config(int width, int height, int start_x, int start_y, std::vector<Rect> obstacles, std::string profile, std::string sprite_player, std::string sprite_obstacle, std::string audio_map, std::string lang) : width(width), height(height), start_x(start_x), start_y(start_y), obstacles(obstacles), profile(profile), sprite_player(sprite_player), sprite_obstacle(sprite_obstacle), audio_map(audio_map), lang(lang) {
}
void Config::add_obstacle(int x, int y) {
  Rect r; r.x = (float)x; r.y = (float)y; r.w = 1.0f; r.h = 1.0f; obstacles.push_back(r);
}
void Config::set_size(int w, int h) {
  width = w; height = h;
}
void Config::set_start(int x, int y) {
  start_x = x; start_y = y;
}
Config Config::load(const std::string& path) {
  Config cfg;
  cfg.width = 40; cfg.height = 12; cfg.start_x = 2; cfg.start_y = 2; cfg.profile = std::string(""); cfg.sprite_player = std::string(""); cfg.sprite_obstacle = std::string(""); cfg.audio_map = std::string(""); cfg.lang = std::string("es");
  std::ifstream f(path);
  if (f) {
  auto trim = [](std::string s)->std::string {
  size_t a=0,b=s.size();
  while (a<b && (s[a]==' '||s[a]=='\t')) a++;
  while (b>a && (s[b-1]==' '||s[b-1]=='\t'||s[b-1]=='\r'||s[b-1]=='\n')) b--;
  return s.substr(a,b-a);
  };
  std::string line;
  while (std::getline(f, line)) {
  auto hash = line.find('#'); if (hash != std::string::npos) line = line.substr(0, hash);
  line = trim(line);
  if (line.empty()) continue;
  if (line.find("width") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.width = std::stoi(trim(line.substr(eq+1))); }
  } else if (line.find("height") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.height = std::stoi(trim(line.substr(eq+1))); }
  } else if (line.find("player_x") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.start_x = std::stoi(trim(line.substr(eq+1))); }
  } else if (line.find("player_y") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.start_y = std::stoi(trim(line.substr(eq+1))); }
  } else if (line.find("sprite_player") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.sprite_player = trim(line.substr(eq+1)); }
  } else if (line.find("sprite_obstacle") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.sprite_obstacle = trim(line.substr(eq+1)); }
  } else if (line.find("profile") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.profile = trim(line.substr(eq+1)); }
  } else if (line.find("audio_map") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.audio_map = trim(line.substr(eq+1)); }
  } else if (line.find("lang") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.lang = trim(line.substr(eq+1)); }
  } else if (line.find("obstacles") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) {
  auto s = line.substr(eq+1);
  std::vector<int> nums; nums.reserve(64);
  std::string cur;
  for (char ch : s) {
  if ((ch>='0'&&ch<='9') || ch=='-') { cur.push_back(ch); }
  else { if (!cur.empty()) { nums.push_back(std::stoi(cur)); cur.clear(); } }
  }
  if (!cur.empty()) { nums.push_back(std::stoi(cur)); cur.clear(); }
  for (size_t i=0; i+1<nums.size(); i+=2) {
  Rect r; r.x = (float)nums[i]; r.y = (float)nums[i+1]; r.w=1.0f; r.h=1.0f; cfg.obstacles.push_back(r);
  }
  }
  }
  }
  }
  if (cfg.width <= 0 || cfg.height <= 0) { cfg.width = 40; cfg.height = 12; Sound::play(std::string("config"), 1, std::string("invalid width/height; defaults applied")); }
  if (!cfg.profile.empty()) {
  std::string pfile = std::string("config.") + cfg.profile + std::string(".toml");
  std::ifstream pf(pfile);
  if (pf) {
  auto trim = [](std::string s)->std::string {
  size_t a=0,b=s.size();
  while (a<b && (s[a]==' '||s[a]=='\t')) a++;
  while (b>a && (s[b-1]==' '||s[b-1]=='\t'||s[b-1]=='\r'||s[b-1]=='\n')) b--;
  return s.substr(a,b-a);
  };
  std::string line;
  while (std::getline(pf, line)) {
  auto hash = line.find('#'); if (hash != std::string::npos) line = line.substr(0, hash);
  line = trim(line);
  if (line.empty()) continue;
  if (line.find("width") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.width = std::stoi(trim(line.substr(eq+1))); } }
  else if (line.find("height") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.height = std::stoi(trim(line.substr(eq+1))); } }
  else if (line.find("player_x") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.start_x = std::stoi(trim(line.substr(eq+1))); } }
  else if (line.find("player_y") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.start_y = std::stoi(trim(line.substr(eq+1))); } }
  else if (line.find("sprite_player") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.sprite_player = trim(line.substr(eq+1)); } }
  else if (line.find("sprite_obstacle") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.sprite_obstacle = trim(line.substr(eq+1)); } }
  else if (line.find("audio_map") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.audio_map = trim(line.substr(eq+1)); } }
  else if (line.find("lang") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.lang = trim(line.substr(eq+1)); } }
  else if (line.find("obstacles") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) {
  auto s = line.substr(eq+1);
  std::vector<int> nums; nums.reserve(64);
  std::string cur;
  for (char ch : s) {
  if ((ch>='0'&&ch<='9') || ch=='-') { cur.push_back(ch); }
  else { if (!cur.empty()) { nums.push_back(std::stoi(cur)); cur.clear(); } }
  }
  if (!cur.empty()) { nums.push_back(std::stoi(cur)); cur.clear(); }
  cfg.obstacles.clear();
  for (size_t i=0; i+1<nums.size(); i+=2) {
  Rect r; r.x = (float)nums[i]; r.y = (float)nums[i+1]; r.w=1.0f; r.h=1.0f; cfg.obstacles.push_back(r);
  }
  }
  }
  }
  }
  }
  return cfg;
}
}
