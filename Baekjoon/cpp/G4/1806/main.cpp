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

int n, s, answer;
vector<int> sequences;

void fn_input() {
    cin >> n >> s;
    for (int i = 0; i < n; ++i) {
        int num; cin >> num;
        sequences.push_back(num);
    }
}

void fn_output() {
    cout << (answer == INF ? 0 : answer) << endl;
}

void fn_solve() {
    answer = INF;

    int left = 0, right = 0;
    int current_sum = sequences[0];

    while (left < n && right < n) {
        if (current_sum >= s) {
            answer = min(answer, right - left + 1);
            current_sum -= sequences[left++];
        }

        else {
            current_sum += sequences[++right];
        }
    }

}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}