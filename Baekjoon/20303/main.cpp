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

int n, m, k, answer;
vector<int> candies;
vector<vector<int>> friends;

void fn_input() {
    cin >> n >> m >> k;

    candies.assign(n + 1, 0);
    for (int i = 1; i <= n; ++i) {
        cin >> candies[i];
    }

    friends.assign(n + 1, vector<int>());
    for (int i = 0; i < m; ++i) {
        int a, b; cin >> a >> b;
        friends[a].push_back(b);
        friends[b].push_back(a);
    }
}

void fn_output() {
    cout << answer << endl;
}

pii get_children_group(int cur_idx, vector<bool> &is_visited) {
    pii ret = { 1, candies[cur_idx] };

    for (int next: friends[cur_idx]) {
        if (!is_visited[next]) {
            is_visited[next] = true;
            pii children_group = get_children_group(next, is_visited);
            ret.first += children_group.first;
            ret.second += children_group.second;
        }
    }

    return ret;
}

void fn_solve() {
    vector<pii> grouped_candies;
    vector<bool> is_visited(n + 1, false);

    for (int i = 1; i <= n; ++i) {
        if (!is_visited[i]) {
            is_visited[i] = true;
            pii grouped_candy = get_children_group(i, is_visited);
            grouped_candies.push_back(grouped_candy);
        }
    }

    vector<int> dp (k + 1, 0);
    for (auto [w, v]: grouped_candies) {
        for (int i = k; i >= 0; --i) {
            if (i - w <= 0) break;

            dp[i] = max(dp[i], dp[i - w] + v);
        }
    }

    answer = dp[k];
}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}