#pragma once
#include <fstream>
#include <iostream>
#include <string>

#include <vector>

using StringInput = std::vector<std::string>;

inline std::string PuzzleFolder = "../puzzle_input/";

inline std::string Pathway(uint day) {
  return PuzzleFolder + "day" + std::to_string(day) + ".txt";
}

inline StringInput read_puzzle_input(uint day) {
  StringInput puzzle_input;
  std::ifstream file(Pathway(day));
  if (!file.is_open()) {
    std::cerr << "Failed to open file" << std::endl;
    return puzzle_input;
  }
  std::string line;
  while (std::getline(file, line)) {
    puzzle_input.push_back(line);
  }
  return puzzle_input;
}

template <typename T> using PuzzleInput = std::vector<T>;