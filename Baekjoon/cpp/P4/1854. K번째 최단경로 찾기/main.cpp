#include <bits/stdc++.h>
#define sync() ios_base::sync_with_stdio(0); cin.tie(0)
#define endl "\n"
#define pb push_back
using namespace std;

typedef pair<int ,int> pii;
typedef vector<pii> vii;

int n, m, k;
vii edge[1001];
priority_queue<int> pq_list[1001];

void solve() {
    pq_list[1].push(0);
    priority_queue<pii> pq; pq.push({ 0, 1 });

    while(!pq.empty()) {
        auto [dist, node] = pq.top(); pq.pop();
        dist = -dist;

        for (auto [next_node, cost]: edge[node]) {
            int next_cost = dist + cost;
            
            if (pq_list[next_node].size() >= k) {
                if (pq_list[next_node].top() <= next_cost) continue;

                pq_list[next_node].pop();
            }

            pq.push({ -next_cost, next_node });
            pq_list[next_node].push(next_cost);
        }
    }
}

int main() {
  sync(); cin >> n >> m >> k;
  for (int i = 0; i < m; ++i) {
    int a, b, c; cin >> a >> b >> c;
    edge[a].pb({ b, c });
  }

  solve();

  for (int i = 1; i <= n; ++i) {
    if (pq_list[i].size() < k) cout << -1 << endl;
    else cout << pq_list[i].top() << endl;
  }
}