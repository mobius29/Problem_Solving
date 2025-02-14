#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
#include <set>
#include <stack>
#include <queue>

#define endl '\n'
#define ends ' '

#define MOD 1000000007
using namespace std;

typedef long long ll;
typedef unsigned long long ull;

const int INF = 0x3F3F3F3F;

const pair<int, int> d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    int n{0}, m{0};
    vector<vector<int>> board;
    vector<vector<int>> finders;
} Input;

typedef struct Output {
    vector<int> answers;
} Output;

Input fn_input() {
    Input input;
    cin >> input.n >> input.m;
    input.board.assign(input.n, vector<int>(input.n, 0));
    input.finders.assign(input.m, vector<int>(4, 0));

    for (int i = 0; i < input.n; ++i) {
        for (int j = 0; j < input.n; ++j) {
            cin >> input.board[i][j];
        }
    }

    for (int i = 0; i < input.m; ++i) {
        for (int j = 0; j < 4; ++j) {
            cin >> input.finders[i][j];
        }
    }

    return input;
}

void fn_output(Output &output) {
    for (int answer: output.answers) {
        cout << answer << endl;
    }
}

vector<int> get_interval_sum(const vector<int> &v) {
    vector<int> ret; ret.push_back(0);

    int sum = 0;
    for (int num: v) {
        sum += num;
        ret.push_back(sum);
    }

    return ret;
}

Output fn_solve(Input &input) {
    Output output;

    vector<vector<int>> interval_sums;
    for (int i = 0; i < input.n; ++i) {
        vector<int> interval_sum = get_interval_sum(input.board[i]);
        interval_sums.push_back(interval_sum);
    }

    for (int i = 0; i < input.m; ++i) {
        vector<int> line = input.finders[i];
        int x1 = line[0], y1 = line[1], x2 = line[2], y2 = line[3];

        int sum = 0;
        for (int x = x1 - 1; x < x2; ++x) {
            int line_sum = interval_sums[x][y2] - interval_sums[x][y1 - 1];
            sum += line_sum;
        }
        output.answers.push_back(sum);
    }

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}