#include <fstream>
#include <string>
#include <sstream>
#include <format>
#include <regex>

#include <stdint.h>

using namespace std;

static const regex invalid_id_regex{R"(^(\d+)\1$)"};
static const regex invalid_id_regex2{R"(^(\d+)\1+$)"};

bool check_product_id(uint64_t product_id)
{
    string id_string = format("{}", product_id);

    // Can rule out ids with uneven length right away.
    if (1 == id_string.length() % 2) {
        return false;
    }

    if (regex_match(id_string, invalid_id_regex))
    {
        return true;
    }
    return false;
}

bool check_product_id2(uint64_t product_id)
{
    string id_string = format("{}", product_id);

    if (regex_match(id_string, invalid_id_regex2))
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

    uint64_t invalid_id_sum = 0;
    uint64_t invalid_id_sum2 = 0;
    uint64_t range_sum, range_count = 0;
    uint64_t range_sum2 = 0;

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
        range_sum2 = 0;

        // Iterate over range.
        for (uint64_t idToCheck = lower; idToCheck <= upper; idToCheck++)
        {
            if (check_product_id(idToCheck))
            {   
                range_count += 1;
                range_sum += idToCheck;
            }
            if (check_product_id2(idToCheck))
            {
                range_sum2 += idToCheck;
            }
        }

        invalid_id_sum += range_sum;
        printf("found %llu for %llu, total %llu.\n", range_count, range_sum, invalid_id_sum);
        invalid_id_sum2 += range_sum2;
    }

    printf("[Part 1] Total sum of invalid ids: %llu\n", invalid_id_sum);
    printf("[Part 2] Total sum of invalid ids: %llu\n", invalid_id_sum2);

    return 0;
}