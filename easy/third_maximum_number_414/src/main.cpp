#include <algorithm>
#include <cstdio>
#include <iostream>
#include <vector>
#include <limits>
#include <cmath>

int thirdMax(std::vector<int> &nums) {
    std::sort(nums.begin(), nums.end());
    auto last = std::unique(nums.begin(), nums.end());
    nums.erase(last, nums.end());

    if(nums.size() < 3) {
	return *(nums.end() - 1);
    }

    return *(nums.end() - 3);
}

int main() {
    std::vector<int> test1 = {3, 2, 1};
    int test1_right_ans = 1;
    int test1_ans = thirdMax(test1);
    std::cout << test1_ans << '\n';
}
