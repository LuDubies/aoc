#include <fstream>
#include <string>
#include <vector>
#include <cstdint>

using namespace std;

typedef pair<uint64_t, uint64_t> range_t;


bool is_in_range(uint64_t id, range_t& range)
{
    return ((id >= range.first) && id <= range.second);
}

/* If the merge is possilbe, resulting range will be stored in range1. */
bool merge_ranges(range_t& range1, range_t& range2)
{
    bool merged = false;

    
    if ((range1.second >= (range2.first - 1)) && (range1.second <= range2.second))
    {
        // range1 ending at/in range2.

        if (range1.first >= range2.first) {
            // range1 fully included in range2.
            range1.first = range2.first;
            range1.second = range2.second;
        }
        else
        {
            // overlap.
            range1.second = range2.second;
        }
        merged = true;
    }
    else if ((range1.first <= range2.second + 1) && range1.first >= range2.first)
    {
        // range1 beginning in/at range2.

        // full inclusion is already covered above, so just adapt the range start.
        range1.first = range2.first;
        merged = true;
    }
    else if ((range1.first <= range2.first) && (range1.second >= range2.second))
    {
        // range2 fully included in range1, merge without changes to range1.
        merged = true;
    }

    return merged;
}

int main()
{
    ifstream file("input.txt");
    string line;

    bool ranges_done = false;
    vector<range_t> ranges = {};

    uint64_t fresh_ingredients = 0;

    while(getline(file, line))
    {
        if (0 == line.size())
        {
            ranges_done = true;
        }
        else
        {
            if (!ranges_done)
            {
                size_t separator_idx = line.find('-');
                ranges.push_back({stoull(line.substr(0, separator_idx)), stoull(line.substr(separator_idx + 1))});
            }
            else
            {
                /* Part 1: Check ingredient IDs. */
                uint64_t incredient_id = stoull(line);
                bool fresh = false;

                for (range_t range: ranges)
                {
                    if (is_in_range(incredient_id, range))
                    {
                        fresh = true;
                        break;
                    }
                }

                if (fresh)
                {
                    fresh_ingredients++;
                }
            }
        }
    }

    printf("[Part 1] Total amount of fresh incrediens: %u\n", fresh_ingredients);

    /* Part 2: Merge the ranges. */

    vector<range_t> processed_ranges = {};
    vector<size_t> processed_ranges_to_cleaup = {};

    while (0 < ranges.size())
    {
        processed_ranges_to_cleaup.clear();

        range_t range = ranges.back();
        ranges.pop_back();

        if (0 == processed_ranges.size())
        {
            // No processed ranges present.
            processed_ranges.push_back(range);
            continue;
        }

        // Check for possible merges with already processed ranges.
        for(size_t ri2 = 0; ri2 < processed_ranges.size(); ri2++)
        {
            if (merge_ranges(range, processed_ranges[ri2]))
            {
                // Ranges were merged into range.
                // Select range that was merged to be removed.
                processed_ranges_to_cleaup.push_back(ri2);
            }
        }

        // Cleanup processed ranges.
        size_t removal_cntr = 0u;
        for (size_t prid: processed_ranges_to_cleaup)
        {
            // Keep counter to adjust ids that need to be removed.
            processed_ranges.erase(processed_ranges.begin() + prid - removal_cntr);
            ++removal_cntr;
        }

        // Add new processed range.
        processed_ranges.push_back(range);
    }

    uint64_t total_fresh_ids = 0u;
    for(range_t prange: processed_ranges)
    {
        total_fresh_ids += prange.second - prange.first + 1;
    }

    printf("[Part 2] Total amount of fresh IDs: %llu\n", total_fresh_ids);

    return 0;
}
