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
namespace Juego {
class EventBus {
public:
  std::map<std::string, std::string> audio_cat;
  std::map<std::string, int> audio_prio;
  EventBus();
  EventBus(std::map<std::string, std::string> audio_cat, std::map<std::string, int> audio_prio);
  void subscribe(std::string event, std::string name);
  void set_audio(std::string event, std::string cat, int prio);
  void emit(std::string event, std::string payload);
};
}
