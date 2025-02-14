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
    vector<int> given_numbers;

    int M {0};
    vector<int> search_numbers;
} Input;

typedef struct Output {
    vector<bool> does_exist;
} Output;

Input fn_input() {
    Input input;

    cin >> input.N;
    for (int i = 0; i < input.N; ++i) {
        int num; cin >> num;
        input.given_numbers.push_back(num);
    }

    cin >> input.M;
    for (int i = 0; i < input.M; ++i) {
        int num; cin >> num;
        input.search_numbers.push_back(num);
    }

    return input;
}

void fn_output(Output &output) {
    for (bool answer: output.does_exist)
        cout << (answer ? 1 : 0) << endl;
}

int binary_search(const vector<int> &list, int key, int N) {
    int left = 0, right = N - 1;

    while (left <= right) {
        int m = (left + right) / 2;

        if (list[m] == key) return m;

        if (list[m] > key) right = m - 1;
        else left = m + 1;
    }

    return -1;
}

Output fn_solve(Input &input) {
    Output output;

    sort(input.given_numbers.begin(), input.given_numbers.end());
    for (int num: input.search_numbers) {
        int result = binary_search(input.given_numbers, num, input.N);
        output.does_exist.push_back(result != -1);
    }

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}

