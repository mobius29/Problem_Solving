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

int N, M, ans, cnt;
vector<int> parent;
priority_queue<ti> pq;

int _find(int x) {
  if(parent[x] == x) return x;

  return parent[x] = _find(parent[x]);
}

void _union(int x, int y) {
  int parent_x = _find(x);
  int parent_y = _find(y);

  if(parent_x == parent_y) return;

  parent[parent_y] = parent_x;
}

void solve() {
  while(!pq.empty()) {
    auto [cost, src, dest] = pq.top(); pq.pop();

    int parent_src = _find(src);
    int parent_dest = _find(dest);

    if(parent_src == parent_dest) continue;

    ans += (-cost);
    _union(src, dest);
  }
}

int main () {
    sync(); cin >> N >> M;
    
    for(int i = 0; i <= N; ++i) {
      parent.push_back(i);
    }

    for(int i = 0; i < M; ++i) {
      int a, b, c; cin >> a >> b >> c;

      if(a == b) continue;
      pq.push({-c, a, b});
    }

    solve();

    cout << ans << endl;

    return 0;
}