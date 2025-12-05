#include <fstream>
#include <string>
#include <vector>
#include <cstdint>

using namespace std;

static int64_t neighbour_offsets[8] = {0, 0, 0, 0, 0, 0, 0, 0};
const static vector<uint8_t> full_nidx = {0, 1, 2, 3, 4, 5, 6, 7};
const static vector<uint8_t> left_nidx = {1, 2, 4, 6, 7};
const static vector<uint8_t> right_nidx = {0, 1, 3, 5, 6};
static size_t line_len = 0;

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

const vector<uint8_t> * nidxs_for_fidx(size_t fidx)
{
    if (0 == ((fidx + 1) % line_len))
    {
        // Field is on the right edge;
        return &right_nidx;
    }
    else if (0 == (fidx % line_len))
    {
        // Field is in the left edge;
        return &left_nidx;
    }
    else
    {
        return &full_nidx;
    }
}

void add_roll(vector<uint8_t>& map, size_t field)
{
    map[field] += 10;

    const vector<uint8_t> * nidxs = nidxs_for_fidx(field);

    for (uint8_t nidx: *nidxs)
    {
        int64_t offset = neighbour_offsets[nidx];
        size_t nfidx = field + offset;
        if (0 <= nfidx && map.size() > nfidx)
        {
            map[nfidx] += 1;
        }
    }
}

void remove_roll(vector<uint8_t>& map, size_t field)
{
    if (map[field] < 10) {
        printf("ERROR: Removal on field %u imposible.\n");
    }
    map[field] -= 10;

    const vector<uint8_t> * nidxs = nidxs_for_fidx(field);

    for (uint8_t nidx: *nidxs)
    {
        int64_t offset = neighbour_offsets[nidx];
        size_t nfidx = field + offset;
        if (0 <= nfidx && map.size() > nfidx)
        {
            if (0 == map[nfidx])
            {
                printf("ERROR: Weight error on field %u. Already 0.\n");
            }
            map[nfidx] -= 1;
        }
    }
}

int main()
{
    ifstream file("input.txt");
    string line;
    
    // Read map.
    vector<uint8_t> map = {};
    size_t field_idx = 0;

    while(getline(file, line))
    {
        // Set neighbour offsets on first line.
        if (0 == neighbour_offsets[0]) {
            line_len = line.size();
            init_neighbout_offsets(line_len);
        }

        // Init map on first line (assume quadratic). Ugly.
        if (0 == map.size()) {
            size_t map_size = line_len * line_len;
            for (size_t fidx = 0; fidx < (line_len * line_len); fidx++)
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

    vector<size_t> rolls_to_remove = {};
    uint64_t accessible_cntr = 1;
    uint64_t total_removed_cntr = 0u;
    uint64_t round_cntr = 1u;

    // Until no rolls can be removed.
    while (0 != accessible_cntr)
    {
        accessible_cntr = 0;

        // Check map for accessible rolls.
        for (size_t ridx = 0; ridx < map.size(); ridx++) {
            uint8_t roll = map[ridx];
            if (roll >= 10 and roll < 14)
            {
                accessible_cntr += 1;
                rolls_to_remove.push_back(ridx);
            }
        }

        if (1 == round_cntr)
        {
            printf("[Part 1] Initially accessible rolls: %u.\n", accessible_cntr);
        }
        printf("Round %u: removing %u rolls.\n", round_cntr, accessible_cntr);

        // Remove the rolls.
        for (size_t ridx: rolls_to_remove)
        {
            remove_roll(map, ridx);
        }
        rolls_to_remove.clear();

        total_removed_cntr += accessible_cntr;
        ++round_cntr;
    }

    printf("[Part 2] Total removable rolls: %u.\n", total_removed_cntr);

    return 0;
}
