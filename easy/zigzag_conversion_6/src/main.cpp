#include <cstddef>
#include <iostream>
#include <string>

std::string convert(std::string s, int numRows) {
    size_t zigzag_size = numRows - 2;
    size_t jump_distance = numRows + zigzag_size;
    auto ptr = s.begin();
    auto str_end = s.end();

    std::string output;
    unsigned int rowCounter = 0;

    // maybe put while here? (for will be nested?)

    for(unsigned int row = 0; row < s.size(); row++) {
	output += *ptr;
	if(rowCounter > 0 && rowCounter <= zigzag_size) {
	    output += *(ptr + numRows + zigzag_size - rowCounter);
	}
	ptr += numRows + zigzag_size;
	rowCounter++;
    }

    return output;
}

int main() {
    std::string input_s = "PAYPALISHIRING";
    int input_numRows = 3;
    std::string result = convert(input_s, input_numRows);

    std::cout << result << '\n';

    return 0;
}
