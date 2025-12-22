#pragma once
#include "reader.h"
#include <utility>
#include <vector>

using Day1T = int;

inline PuzzleInput<Day1T> parse_day_1() {
  StringInput to_be_parsed = read_puzzle_input(1);
  PuzzleInput<Day1T> parsed;
  for (const auto &line : to_be_parsed) {
    char direction = line[0];
    int value = std::stoi(line.substr(1));
    parsed.push_back((direction == 'R') ? -value : value);
  }
  return parsed;
}

using Day3T = std::vector<uint8_t>;
inline PuzzleInput<Day3T> parse_day_3() {
  StringInput to_be_parsed = read_puzzle_input(3);
  PuzzleInput<Day3T> parsed;
  for (const auto &line : to_be_parsed) {
    std::vector<uint8_t> joltage;
    for (char c : line)
      joltage.push_back(c - '0');
    parsed.push_back(joltage);
  }
  return parsed;
}
