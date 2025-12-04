#include <fstream>
#include <string>
#include <vector>
#include <cstdint>

using namespace std;

static int64_t neighbour_offsets[8] = {0, 0, 0, 0, 0, 0, 0, 0};

/* 
* Initialize the offsets of the 0 neighbours for given line length.
*/
void init_neighbout_offsets(size_t line_len)
{
    neighbour_offsets[0] = -line_len - 1;
    neighbour_offsets[1] = -line_len;
    neighbour_offsets[2] = -line_len + 1;
    neighbour_offsets[3] = -1;
    neighbour_offsets[4] = 1;
    neighbour_offsets[5] = line_len - 1;
    neighbour_offsets[6] = line_len;
    neighbour_offsets[7] = line_len + 1;
}

void add_roll(vector<uint8_t>& map, size_t field)
{
    map[field] += 10;

    for (int64_t offset: neighbour_offsets)
    {
        int64_t nidx = field + offset;
        if (0 <= nidx && map.size() > nidx)
        {
            map[nidx] += 1;
        }
    }
}

void remove_roll(vector<uint8_t>& map, size_t field)
{
    if (map[field] < 10) {
        printf("ERROR: Removal on field %u imposible.");
    }
    map[field] -= 10;
}

int main()
{
    ifstream file("testinput.txt");
    string line;
    
    // Read map.
    vector<uint8_t> map = {};
    size_t field_idx = 0;

    while(getline(file, line))
    {
        // Set neighbour offsets on first line.
        if (0 == neighbour_offsets[0]) {
            init_neighbout_offsets(line.size());
        }

        // Init map on first line (assume quadratic). Ugly.
        if (0 == map.size()) {
            size_t map_size = line.size() * line.size();
            for (size_t fidx = 0; fidx < (line.size() * line.size()); fidx++)
            {
                map.push_back(0);
            }
        }

        for (char c: line) {
            if ('@' == c)
            {
                add_roll(map, field_idx);
            }
            ++field_idx;
        }
    }

    for (size_t fidx = 0; fidx < map.size(); fidx++)
    {
        printf("%u ", map[fidx]);
        if ((fidx + 1) % 10 == 0) {
            printf("\n");
        }
    }

    // Check map for accessible rolls.
    uint64_t accessible_count = 0;

    for (uint8_t roll: map) {
        if (roll >= 10 and roll < 14)
        {
            accessible_count += 1;
        }
    }

    printf("[Part 1] Initially accessible rolls: %u.", accessible_count);

    return 0;
}
