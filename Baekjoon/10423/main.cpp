#include <bits/stdc++.h>
#define sync() ios_base::sync_with_stdio(0); cin.tie(0)
#define endl "\n"
#define ends " "
#define pb push_back
#define all(x) (x).begin(), (x).end()
#define getx(x, i) get<(x)>(i)
using namespace std;

typedef long long ll;
typedef pair<int ,int> pii;
typedef pair<ll, ll> pll;
typedef vector<int>  vi;
typedef vector<ll> vl;
typedef vector<pii> vii;
typedef vector<pll> vll;
typedef tuple<int, int, int> ti;
typedef tuple<ll, ll, ll> tl;

const int INF = 0x3f3f3f3f;
const ll LINF = 0x3f3f3f3f3f3f3f3f;

const int dx[] = {-1, 0, 1, 0};
const int dy[] = {0, -1, 0, 1};

int N, M, K, ans;
int parent[1001];
bool plant[1001];
priority_queue<ti> pq;

int _find(int x) {
  if(parent[x] == x) return x;

  return parent[x] = _find(parent[x]);
}

void solve();

int main () {
    sync();
    cin >> N >> M >> K;

    for(int i = 1; i <= N; ++i) {
      parent[i] = i;
    }

    for(int i = 0; i < K; ++i) {
      int plant_num = -1; cin >> plant_num;

      plant[plant_num] = true;
    }

    for(int i = 0; i < M; ++i) {
      int u = -1, v = -1, w = -1; cin >> u >> v >> w;

      pq.push({-w, u, v});
    }

    solve();

    cout << ans << endl;

    return 0;
}

void solve() {
  while(!pq.empty()) {
    auto [w, u, v] = pq.top();
    pq.pop();

    if(plant[u] && plant[v]) continue;

    int find_u = _find(u);
    int find_v = _find(v);

    if(find_u == find_v || (plant[find_u] && plant[find_v])) continue;

    if(plant[find_u]) 
      parent[find_v] = find_u;
    else
      parent[find_u] = find_v;
    
    ans += (-w);
  }
}
