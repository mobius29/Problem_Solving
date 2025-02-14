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

void dfs(const vector<int> &list, const int M, vector<int> &cur_list, int cur_idx, vector<vector<int>> &answer) {
    if (cur_list.size() == M) {
        answer.emplace_back(cur_list.begin(), cur_list.end());
        return ;
    }

    for (int i = cur_idx; i < list.size(); ++i) {
        int num = list[i];
        cur_list.push_back(num);
        dfs(list, M, cur_list, i, answer);
        cur_list.pop_back();
    }
}


Output fn_solve(Input &input) {
    Output output;

    set<int> s;
    for (int num: input.numbers) {
        s.insert(num);
    }

    vector<int> list(s.begin(), s.end());

    vector<int> cur_list;
    dfs(list, input.m, cur_list, 0, output.permutations);

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}

