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

int n, m, answer;
vector<int> memories;
vector<int> costs;

void fn_input() {
    cin >> n >> m;

    for (int i = 0; i < n; ++i) {
        int memory; cin >> memory;
        memories.push_back(memory);
    }

    for (int i = 0; i < n; ++i) {
        int cost; cin >> cost;
        costs.push_back(cost);
    }
}

void fn_output() {
    cout << answer << endl;
}

int fn_get_sum(const vector<int> &list) {
    int sum = 0;
    for (int item: list) sum += item;

    return sum;
}

void fn_solve() {
    vector<int> dp(m + 1, INF);

    for (int i = 0; i < n; ++i) {
        int w = memories[i], c = costs[i];

        for (int j = m; j >= 0; --j) {
            if (j - w <= 0)
                dp[j] = min(dp[j], c);

            else
                dp[j] = min(dp[j], dp[j - w] + c);
        }
    }

    answer = dp[m];
}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}