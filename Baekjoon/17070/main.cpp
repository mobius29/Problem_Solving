#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>

#define endl '\n'
#define ends ' '

using namespace std;

typedef long long ll;
typedef unsigned long long ull;

const int INF = 0x3F3F3F3F;

const pair<int, int> d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    int n{0};
    vector<vector<int>> room;
} Input;

typedef struct Output {
    int answer;
} Output;

Input fn_input() {
    Input input;

    cin >> input.n;

    input.room.assign(input.n + 1, vector<int>(input.n + 1, 0));
    for (int i = 1; i <= input.n; ++i)
        for (int j = 1; j <= input.n; ++j)
            cin >> input.room[i][j];

    return input;
}

void fn_output(Output &output) {
    cout << output.answer << endl;
}

Output fn_solve(Input &input) {
    Output output;

    vector<vector<int>> horizontal(input.n + 1, vector<int>(input.n + 1, 0));
    vector<vector<int>> vertical(input.n + 1, vector<int>(input.n + 1, 0));
    vector<vector<int>> diagonal(input.n + 1, vector<int>(input.n + 1, 0));

    horizontal[1][2] = 1;
    for (int i = 1; i <= input.n; ++i) {
        for (int j = 3; j <= input.n; ++j) {
            if (input.room[i][j] == 1) continue;
            horizontal[i][j] = horizontal[i][j-1] + diagonal[i][j-1];
            vertical[i][j] = vertical[i-1][j] + diagonal[i-1][j];

            if (input.room[i-1][j] == 1 || input.room[i][j-1] == 1) continue;
            diagonal[i][j] = horizontal[i-1][j-1] + vertical[i-1][j-1] + diagonal[i-1][j-1];
        }
    }

    output.answer = horizontal[input.n][input.n] + vertical[input.n][input.n] + diagonal[input.n][input.n];

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}