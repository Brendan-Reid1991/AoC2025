#pragma once
#include "reader.h"
#include <utility>

using Day1Line = std::pair<char, int>;

inline PuzzleInput<Day1Line> parse_day_1() {
  StringInput to_be_parsed = read_puzzle_input(1);
  PuzzleInput<Day1Line> parsed;
  for (const auto &line : to_be_parsed) {
    char direction = line[0];
    int value = std::stoi(line.substr(1));
    parsed.push_back({direction, value});
  }
  return parsed;
}
