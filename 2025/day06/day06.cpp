#include <fstream>
#include <string>
#include <cstdint>
#include <format>
#include <regex>


using namespace std;

static const regex num_w_leading_ws_regex{R"(\s*(\d+))"};
static const regex operand_w_leading_ws_regex{R"(\s*([\*\+]))"};

int main()
{
    ifstream file("input.txt");
    string line;

    vector<vector<uint64_t>> problems = {};
    vector<char> operators = {};

    vector<string> lines = {}; // Part 2

    // Parse problem.
    while(getline(file, line))
    {
        lines.push_back(line);

        vector<uint64_t> operands = {};

        for (smatch sm; regex_search(line, sm, num_w_leading_ws_regex);)
        {
            operands.push_back(stoul(sm.str()));
            line = sm.suffix();
        }

        if (0 != operands.size())
        {
            problems.push_back(operands);
        }
        else
        {
            for (smatch sm; regex_search(line, sm, operand_w_leading_ws_regex);)
            {
                operators.push_back(sm.str(1)[0]);
                line = sm.suffix();
            }
        }
    }

    printf("Parsed %u lines of len %u. Parsed %u operands.\n", problems.size(), problems[0].size(), operators.size());

    /* Part 1 */

    uint64_t problem_sum = 0;

    for (size_t problem_id = 0; problem_id < problems[0].size(); problem_id++)
    {
        uint64_t problem_solution;
        char op = operators[problem_id];

        if ('*' == op)
        {
            problem_solution = 1;
        }
        else
        {
            problem_solution = 0;
        }

        for (size_t operand_id = 0; operand_id < problems.size(); operand_id++)
        {
            if ('*' == op)
            {
                problem_solution *= problems[operand_id][problem_id];
            }
            else
            {
                problem_solution += problems[operand_id][problem_id];
            }
        }

        printf("Problem %u (%c) solution: %llu\n", problem_id, op, problem_solution);
        problem_sum += problem_solution;
    }

    printf("[Part 1] Total sum of solutions: %llu\n", problem_sum);

    /* Part 2 */

    lines.pop_back();  // Dont need the operators again.

    uint64_t problem_sum_2 = 0;
    uint64_t problem_solution_2 = 0;
    size_t problem_id = 0;
    vector<uint64_t> problem_operands = {};

    for (size_t ci = 0; ci < lines[0].size(); ci++)
    {
        // Extract column from line.
        vector<char> column = {};
        for (size_t li = 0; li < lines.size(); li++)
        {
            column.push_back(lines[li][ci]);
        }
        string column_str = string(column.begin(), column.end());

        if (column_str.find_first_not_of(' ') == string::npos)
        {
            // Empty columns means we can calculate the problem.

            char op = operators[problem_id];

            if ('*' == op)
            {
                problem_solution_2 = 1;
            }
            else
            {
                problem_solution_2 = 0;
            }

            for (size_t operand_id = 0; operand_id < problem_operands.size(); operand_id++)
            {
                if ('*' == op)
                {
                    problem_solution_2 *= problem_operands[operand_id];
                }
                else
                {
                    problem_solution_2 += problem_operands[operand_id];
                }
            }

            printf("Problem %u (%c) solution: %llu\n", problem_id, op, problem_solution_2);
            problem_sum_2 += problem_solution_2;
            ++problem_id;
            problem_operands.clear();
        }
        else
        {
            // Non empty line can be parsed as an operand.
            problem_operands.push_back(stoull(column_str));
        }
    }

    // Parse the last problem.

    char op = operators[problem_id];

    if ('*' == op)
    {
        problem_solution_2 = 1;
    }
    else
    {
        problem_solution_2 = 0;
    }

    for (size_t operand_id = 0; operand_id < problem_operands.size(); operand_id++)
    {
        if ('*' == op)
        {
            problem_solution_2 *= problem_operands[operand_id];
        }
        else
        {
            problem_solution_2 += problem_operands[operand_id];
        }
    }
    printf("Problem %u (%c) solution: %llu\n", problem_id, op, problem_solution_2);
    problem_sum_2 += problem_solution_2;

    printf("[Part 2] Total sum of solutions: %llu\n", problem_sum_2);

    return 0;
}