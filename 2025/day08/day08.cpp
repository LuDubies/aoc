#include <fstream>
#include <string>
#include <cstdint>
#include <vector>
#include <regex>
#include <tuple>
#include <cmath>
#include <set>

#define CONNECTION_THRESHOLD 1000u
#define INFILE "input.txt"

using namespace std;

typedef tuple<uint64_t, uint64_t, uint64_t> jbox;  // x, y, z.
static const regex jbox_regex{R"(^(\d+),(\d+),(\d+)$)"};
typedef tuple<long double, uint64_t, uint64_t> dist;  // distance, d1 id, d2 id.

bool is_smaller_dist(dist& d1, dist& d2)
{
    return get<0>(d1) < get<0>(d2);
}

dist jbox_distance(uint64_t j1_id, jbox& j1, uint64_t j2_id, jbox& j2)
{
    long double d = sqrt(   (get<0>(j1) - get<0>(j2)) * (get<0>(j1) - get<0>(j2)) +
                            (get<1>(j1) - get<1>(j2)) * (get<1>(j1) - get<1>(j2)) +
                            (get<2>(j1) - get<2>(j2)) * (get<2>(j1) - get<2>(j2)));
    return {d, j1_id, j2_id};
}

void print_dist(dist& d)
{
    printf("Dist %llu - %llu: %Lf\n", get<1>(d), get<2>(d), get<0>(d));
}

void print_jbox(uint64_t jid, jbox& j)
{
    printf("Jbox %llu: %llu, %llu, %llu\n", jid, get<0>(j), get<1>(j), get<2>(j));
}

int64_t get_circuit_for_jbox(uint64_t jbox_id, vector<set<uint64_t>>& circuits)
{
    for (size_t ci = 0u; ci < circuits.size(); ci++)
    {
        set<uint64_t> circuit = circuits[ci];
        if (circuit.contains(jbox_id))
        {
            return ci;
        }
    }

    return -1;
}


int main()
{
    ifstream file(INFILE);
    string line;

    vector<jbox> jboxes = {};
    set<dist> distances = {};

    vector<set<uint64_t>> circuits = {};

    // Read in junction boxes.
    while (getline(file, line))
    {
        smatch sm;
        regex_match(line, sm, jbox_regex);
        jboxes.push_back({stoul(sm.str(1)), stoul(sm.str(2)), stoul(sm.str(3))});
    }

    // Build set of distances.
    for (size_t oi = 0u; oi < jboxes.size() - 1u; oi++)
    {
        for (size_t ii = oi+1; ii < jboxes.size(); ii++)
        {
            distances.insert(jbox_distance(oi, jboxes[oi], ii, jboxes[ii]));
        }
    }

    printf("We have %llu distances.\n", distances.size());

    uint16_t connections_made = 0u;

    for (dist d: distances)
    {
        printf("Connection %4u - ", connections_made + 1);
        print_dist(d);
        uint64_t jid1 = get<1>(d);
        uint64_t jid2 = get<2>(d);

        int64_t j1_circ = get_circuit_for_jbox(jid1, circuits);
        int64_t j2_circ = get_circuit_for_jbox(jid2, circuits);

        // Check if the jids are already in any circuit.
        
        if (-1 == j1_circ && -1 == j2_circ)
        {
            // They are both unconnected, form new circuit.
            set<uint64_t> new_circuit = {jid1, jid2};
            circuits.push_back(new_circuit);
        }
        else if (j1_circ == j2_circ)
        {
            // Both in the same circuit already. Do nothing. Skip connections increase!!!
        }
        else if (-1 == j1_circ && -1 != j2_circ)
        {
            // Add j1 to j2s circuit.
            circuits[j2_circ].insert(jid1);
        }
        else if (-1 != j1_circ && -1 == j2_circ)
        {
            // Add j2 to j1s circuit.
            circuits[j1_circ].insert(jid2);
        }
        else
        {
            auto c1 = circuits[j1_circ];
            auto c2 = circuits[j2_circ];
            c1.insert(c2.begin(), c2.end());
            circuits[j1_circ] = c1;
            circuits.erase(circuits.begin()+j2_circ);
        }

        connections_made++;

        if (connections_made == CONNECTION_THRESHOLD)
        {
            // Evaluate Part 1.
            printf("Made %lu connections.\n", connections_made);
            vector<size_t> circuit_sizes = {};
            for (auto circ: circuits)
            {
                circuit_sizes.push_back(circ.size());
            }
            sort(circuit_sizes.begin(), circuit_sizes.end(), greater<size_t>());

            uint64_t solution = circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2];

            printf("[Part 1] Solution is: %llu.\n", solution);
        }

        if (circuits[0].size() == jboxes.size())
        {
            // Evaluate Part 2.
            jbox j1 = jboxes[jid1];
            jbox j2 = jboxes[jid2];

            uint64_t solution = get<0>(j1) * get<0>(j2);

            printf("[Part 2] Solution is: %llu.\n", solution);
            break;
        }
    }

    printf("Made %lu connections.\n", connections_made);

    return 0;
}