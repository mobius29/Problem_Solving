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
    set<int> numbers;
} Input;

typedef struct Output {
    vector<vector<int>> permutations;
} Output;

Input fn_input() {
    Input input;

    cin >> input.n >> input.m;
    for (int i = 0; i < input.n; ++i) {
        int number; cin >> number;
        input.numbers.insert(number);
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

void dfs(const vector<int> &list, vector<int> &cur_list, int cur_idx, int M, vector<vector<int>> &answer) {
    if (cur_list.size() == M) {
        answer.emplace_back(cur_list.begin(), cur_list.end());
        return ;
    }

    for (int i = cur_idx; i < list.size(); ++i) {
        cur_list.push_back(list[i]);
        dfs(list, cur_list, i, M, answer);
        cur_list.pop_back();
    }
}


Output fn_solve(Input &input) {
    Output output;

    vector<int> list(input.numbers.begin(), input.numbers.end());

    vector<int> cur_list;
    dfs(list, cur_list, 0, input.m, output.permutations);

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}

