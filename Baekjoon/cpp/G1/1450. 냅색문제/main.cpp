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

int N, C, ans;
vector<int> stuff;

void _dfs(int cur, int end, int sum, vector<int>& v);

int main () {
    sync();
    cin >> N >> C;

    for(int i = 0; i < N; ++i) {
      int w; cin >> w;
      stuff.pb(w);
    }

    vector<int> stuff1, stuff2;
    _dfs(0, N/2 + 1, 0, stuff1); sort(all(stuff1));
    _dfs(N/2 + 1, N, 0, stuff2); sort(all(stuff2));

    for(int i = 0; i < stuff1.size(); ++i) {
      int e = stuff2.size() - 1;

      while(e >= 0 && (stuff1[i] + stuff2[e] > C)) --e;

      ans += e+1;
    }

    cout << ans << endl;
    

    return 0;
}

void _dfs(int cur, int end, int sum, vector<int>& v) {
  if(sum > C) return ;

  if(cur == end) {
    v.pb(sum);
    return;
  }

  _dfs(cur + 1, end, sum, v);
  _dfs(cur + 1, end, sum + stuff[cur], v);
}