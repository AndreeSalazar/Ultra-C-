#pragma once
#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  ifdef _MSC_VER
#    define UCPP_NOEXCEPT noexcept
#  else
#    define UCPP_NOEXCEPT noexcept
#  endif
#endif
#include "pch.hpp"
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
class UCPP_API EventBus {
public:
  std::map<std::string, std::string> audio_cat;
  std::map<std::string, int> audio_prio;
  EventBus();
  EventBus(std::map<std::string, std::string> audio_cat, std::map<std::string, int> audio_prio);
  void subscribe(const std::string& event, const std::string& name);
  void set_audio(const std::string& event, const std::string& cat, int prio);
  void emit(const std::string& event, const std::string& payload);
};
}
