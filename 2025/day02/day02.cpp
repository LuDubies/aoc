#include <fstream>
#include <string>
#include <sstream>
#include <format>
#include <regex>

#include <stdint.h>

using namespace std;

static const regex invalidIdRegex{R"(^(\d+)\1$)"};

bool check_product_id(uint64_t productId)
{
    string idString = format("{}", productId);

    // Can rule out ids with uneven length right away.
    if (1 == idString.length() % 2) {
        return false;
    }

    if (regex_match(idString, invalidIdRegex))
    {
        return true;
    }
    return false;
}

int main()
{
    // Read single line input.
    ifstream file("input.txt");
    string line, range;
    getline(file, line);

    uint64_t invalidIdSum = 0;
    uint64_t range_sum, range_count = 0;

    // Vars for parsing ranges.
    uint64_t lower, upper;
    uint32_t dashIdx;

    // Split into ranges.
    istringstream linestream(line);
    while(getline(linestream, range,','))
    {
        dashIdx = range.find('-');
        lower = stoull(range.substr(0, dashIdx));
        upper = stoull(range.substr(dashIdx+1));

        printf("Checking %u-%u      ", lower, upper);
        range_sum = 0;
        range_count = 0;

        // Iterate over range.
        for (uint64_t idToCheck = lower; idToCheck <= upper; idToCheck++)
        {
            if (check_product_id(idToCheck))
            {   
                range_count += 1;
                range_sum += idToCheck;
            }
        }

        invalidIdSum += range_sum;
        printf("found %u for %u, total %u.\n", range_count, range_sum, invalidIdSum);
    }

    printf("Total sum of invalid ids: %u\n", invalidIdSum);

    return 0;
}