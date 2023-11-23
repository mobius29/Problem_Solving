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
    vector<int> cards;
    bool checked[1000001];
} Input;

typedef struct Output {
    vector<int> scores;
} Output;

Input input;
Output output;

void fn_input() {
    cin >> input.n;

    for (int i = 0; i < input.n; ++i) {
        int card; cin >> card;
        input.cards.push_back(card);
        input.checked[card] = true;
    }
}

void fn_output() {
    for (int card: input.cards) {
        cout << output.scores[card] << ends;
    }
    cout << endl;
}

void fn_solve() {
    output.scores.assign(1000001, 0);

    for (int card: input.cards) {
        for (int j = 2; card * j <= 1000000; ++j) {
            if (input.checked[card * j]) {
                output.scores[card] += 1;
                output.scores[card * j] -= 1;
            }
        }
    }
}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}