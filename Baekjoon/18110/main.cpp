#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>

#define endl '\n'
#define ends ' '
using namespace std;

const int INF = 0x3F3F3F3F;

const pair<int, int> d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    int n {0};
    vector<int> difficulties;
} Input;

typedef struct Output {
    int difficulty;
} Output;

Input fn_input() {
    Input input;

    cin >> input.n;
    for (int i = 0; i < input.n; ++i) {
        int difficulty; cin >> difficulty;
        input.difficulties.push_back(difficulty);
    }

    return input;
}

void fn_output(Output &output) {
    cout << output.difficulty << endl;
}

Output fn_solve(Input &input) {
    Output output;

    if (input.n == 0) {
        output.difficulty = 0;
        return output;
    }

    sort(input.difficulties.begin(), input.difficulties.end());

    double sum = 0;

    int cut = (int)round((double)input.n * 15 / 100);
    for (int i = cut; i < input.n - cut; ++i) {
        sum += input.difficulties[i];
    }

    output.difficulty = (int)round(sum / (input.n - 2 * cut));

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}

