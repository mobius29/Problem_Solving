#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
#include <set>

#define endl '\n'
#define ends ' '
using namespace std;

const int INF = 0x3F3F3F3F;

const pair<int, int> d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    int n = 0, m = 0;
    vector<int> numbers;
} Input;

typedef struct Output {
    vector<vector<int>> permutations;
} Output;

Input fn_input() {
    Input input;

    cin >> input.n >> input.m;
    for (int i = 0; i < input.n; ++i) {
        int number; cin >> number;
        input.numbers.push_back(number);
    }

    return input;
}

void fn_output(Output &output) {
    for (vector<int> &permutation: output.permutations) {
        for (auto number: permutation) {
            cout << number << ends;
        }
        cout << endl;
    }
}

void dfs(const vector<int> &list, vector<int> &counts, vector<int> &cur_list, int M, vector<vector<int>> &answer) {
    if (cur_list.size() == M) {
        answer.emplace_back(cur_list.begin(), cur_list.end());
        return ;
    }

    for (int i = 0; i < list.size(); ++i) {
        int num = list[i];

        if (counts[num] <= 0) continue;

        cur_list.push_back(num);
        counts[num] -= 1;

        dfs(list, counts, cur_list, M, answer);

        counts[num] += 1;
        cur_list.pop_back();
    }
}


Output fn_solve(Input &input) {
    Output output;

    vector<int> counts(10001, 0);
    set<int> s;

    for (int num: input.numbers) {
        counts[num] += 1;
        s.insert(num);
    }

    vector<int> list(s.begin(), s.end());

    vector<int> cur_list;
    dfs(list, counts, cur_list, input.m, output.permutations);

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}

