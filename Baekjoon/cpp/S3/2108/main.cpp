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
    int N {0};
    vector<int> numbers;
} Input;

typedef struct Output {
    int average;
    int median;
    int mode;
    int range;
} Output;

Input fn_input() {
    Input input; cin >> input.N;

    input.numbers.assign(input.N, 0);
    for (int i = 0; i < input.N; ++i) {
        cin >> input.numbers[i];
    }

    return input;
}

void fn_output(Output &output) {
    cout << output.average << endl;
    cout << output.median << endl;
    cout << output.mode << endl;
    cout << output.range << endl;
}

Output fn_solve(Input &input) {
    Output output;

    vector<pair<int, int>> counts;
    for (int i = -4000; i <= 4000; ++i) counts.emplace_back(0, i);

    int sum = 0;
    sort(input.numbers.begin(), input.numbers.end());
    for (int i = 0; i < input.N; ++i) {
        int num = input.numbers[i];
        sum += num;
        counts[num + 4000].first -= 1;
    }
    sort(counts.begin(), counts.end());

    output.average = (int)round(sum / (double) input.N);
    output.median = input.numbers[input.N / 2];
    output.range = input.numbers[input.N - 1] - input.numbers[0];
    output.mode = counts[0].first == counts[1].first ? counts[1].second : counts[0].second;

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}

