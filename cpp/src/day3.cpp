#include "parsers.h"
#include <algorithm>
#include <iostream>
#include <vector>

long max_joltages(const std::vector<uint8_t> &bank, int turn_on = 2);

int main() {
  int batteries = 12;
  PuzzleInput<Day3T> joltage = parse_day_3();
  long total_output_joltages = 0;
  for (std::vector<uint8_t> bank : joltage) {
    total_output_joltages += max_joltages(bank, batteries);
  }
  std::cout << total_output_joltages << std::endl;
  return total_output_joltages;
}

auto max_joltage_in_index_range(const std::vector<uint8_t> &bank, int start,
                                int end) {
  return std::max_element(bank.begin() + start, bank.begin() + end);
}

long max_joltages(const std::vector<uint8_t> &bank, int turn_on) {
  std::vector<uint8_t> batteries(turn_on);
  int start_at = 0;
  int end_at = bank.size() - (turn_on - 1);

  for (int i = 0; i < turn_on; i++) {
    auto _max = max_joltage_in_index_range(bank, start_at, end_at);
    batteries[i] = *_max;
    start_at = std::distance(bank.begin(), _max) + 1;
    end_at += 1;
  }
  long result = 0;
  for (uint8_t digit : batteries) {
    result = result * 10 + digit;
  };
  return result;
}
