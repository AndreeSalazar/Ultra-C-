#include "pch.hpp"
#include "eventbus.hpp"
#include "sound.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
EventBus::EventBus() : audio_cat(std::map<std::string, std::string>()), audio_prio(std::map<std::string, int>()) {}
EventBus::EventBus(std::map<std::string, std::string> audio_cat, std::map<std::string, int> audio_prio) : audio_cat(audio_cat), audio_prio(audio_prio) {
}
void EventBus::subscribe(const std::string& event, const std::string& name) {
  std::cout << std::string("Event: ") + event + std::string(" payload=") + name << std::endl;
}
void EventBus::set_audio(const std::string& event, const std::string& cat, int prio) {
  audio_cat[event] = cat; audio_prio[event] = prio;
}
void EventBus::emit(const std::string& event, const std::string& payload) {
  std::string cat = "events";
  int pr = 5;
  auto itc = audio_cat.find(event);
  if (itc != audio_cat.end()) { cat = itc->second; }
  auto itp = audio_prio.find(event);
  if (itp != audio_prio.end()) { pr = itp->second; }
  Sound::play(cat, pr, payload);
}
}
