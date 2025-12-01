#include <fstream>
#include <string>
#include <stdint.h>

using namespace std;

int main() {

    ifstream file("input.txt");
    string line;

    uint32_t dial = 50;  // Init dial to 50.
    uint32_t zero_end_counter = 0;  // Counter for 0 positions.
    uint32_t zero_pass_counter = 0;  // Counter for 0 passes.
    uint32_t steps = 0;  // Steps for each rotation

    while (getline(file, line))
    {
        steps = stoi(line.substr(1));

        // Turn dial left or right.
        if (0 == line.substr(0, 1).compare("L"))
        {
            zero_pass_counter += (100 + steps - dial) / 100;
            zero_pass_counter = 0 == dial ? zero_pass_counter - 1 : zero_pass_counter;  // Correct for left turn from 0;
            dial += (100 - (steps % 100));
            dial %= 100;
        }
        else if (0 == line.substr(0, 1).compare("R"))
        {
            dial += steps;
            zero_pass_counter += dial / 100;
            dial %= 100;
        }
        else
        {
            printf("Parsing Error!");
            return 1;
        }
        
        if (!dial) {
            zero_end_counter++;
        }
        printf("%s: Dial at %i, so far %u zeroes direct, %u zeroes passed.\n", line.c_str(), dial, zero_end_counter, zero_pass_counter);
    }

    printf("In total 0 was stopped at %u times, %u times passed.\n", zero_end_counter, zero_pass_counter);

    return 0;
}