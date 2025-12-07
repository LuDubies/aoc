#include <fstream>
#include <string>
#include <cstdint>
#include <vector>

using namespace std;

int main()
{
    ifstream file("input.txt");
    string line;

    vector<vector<uint64_t>> tbeams = {};
    vector<vector<bool>> splitters = {};

    // Parse manual.
    while(getline(file, line))
    {
        if (0u == tbeams.size())
        {
            // Parse tbeam start and add empty splitter line.
            size_t sidx = line.find_first_of('S');
            vector<uint64_t> beam_line = vector<uint64_t>(line.size(), 0u);
            beam_line[sidx] = 1u;
            tbeams.push_back(beam_line);
            splitters.push_back(vector<bool>(line.size(), false));
        }
        else
        {
            // Add empty beam line and parse splitters.
            tbeams.push_back(vector<uint64_t>(line.size(), 0u));
            vector<bool> splitter_line = vector<bool>(line.size(), false);
            size_t soffs = line.find_first_of('^');
            size_t suboffs = soffs;
            while (suboffs != string::npos)
            {
                splitter_line[soffs] = true;
                line = line.substr(suboffs + 1u);
                suboffs = line.find_first_of('^');
                soffs += suboffs + 1;
            }
            splitters.push_back(splitter_line);
        }
    }

    // Simulate tbeam path.

    uint64_t split_count = 0u;

    for (size_t row = 1; row < splitters.size(); row++)
    {
        for (size_t col = 0; col < splitters[0].size(); col++)
        {
            // Check if a  beam is present in the row above.
            uint64_t timelines_above = tbeams[row - 1u][col];
            if (0u < timelines_above)
            {
                if (!splitters[row][col])
                {
                    // No splitter is here, the beam passes on downward.
                    tbeams[row][col] += timelines_above;
                }
                else
                {
                    // Splitter here, the beam is split to the left and right, split timelines above.
                    // No splitters in first and last column, so skip boundary check of col!
                    tbeams[row][col - 1u] += timelines_above;
                    tbeams[row][col + 1u] += timelines_above;
                    ++split_count;
                }
            }
        }
    }

    printf("[Part 1] Total beam splits: %llu\n", split_count);

    // Part 2: Count timelines at the bottom.

    uint64_t total_timelines = 0u;

    for (uint64_t timelines: tbeams[tbeams.size() - 1u])
    {
        total_timelines += timelines;
    }

    printf("[Part 2] Total timelines: %llu\n", total_timelines);

    return 0;
}