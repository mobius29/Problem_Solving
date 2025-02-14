#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>

#define endl '\n'
#define ends ' '

using namespace std;


typedef long long ll;
typedef pair<int, int> pii;
typedef pair<ll, ll> pll;

const int INF = 0x3F3F3F3F;

const int dx[4] = { -1, 0, 1, 0 };
const int dy[4] = { 0, 1, 0, -1 };

typedef struct Input {
    int n;
    vector<int> solutions;
} Input;

typedef struct Output {
    int answer[2];
} Output;

Input input;
Output output;

void fn_input() {
    cin >> input.n;
    for(int i = 0; i < input.n; ++i) {
        int solution; cin >> solution;
        input.solutions.push_back(solution);
    }
}

void fn_output() {
    cout << output.answer[0] << ends << output.answer[1] << endl;
}

void fn_solve() {
    int minimum_value = 2e9;
    int left = 0, right = input.n - 1;

    while (left < right) {
        int mixed_solution = input.solutions[left] + input.solutions[right];

        if (abs(mixed_solution) < minimum_value) {
            minimum_value = abs(mixed_solution);
            output.answer[0] = input.solutions[left];
            output.answer[1] = input.solutions[right];
        }

        if (mixed_solution == 0) break;
        if (mixed_solution > 0) right -= 1;
        if (mixed_solution < 0)left += 1;
    }
}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}