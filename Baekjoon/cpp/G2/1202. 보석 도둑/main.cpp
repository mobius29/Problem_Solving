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

ll N, K, ans;
vector<pii> jewel;
vector<int> C;

void init() {
  cin >> N >> K;

  for(int i = 0; i < N; ++i) {
    int a, b; cin >> a >> b;
    jewel.pb({a, b});
  }

  for(int i = 0; i < K; ++i){
    int c; cin  >> c;
    C.pb(c);
  }

  sort(all(jewel));
  sort(all(C));
}

void solve();

int main () {
    sync(); 

    init();
    solve();

    cout << ans << endl;
    return 0;
}

void solve() {
  int x = 0;
  priority_queue<int> value;

  for(int i = 0; i < K; ++i) {
    while((x < N) && (jewel[x].first <= C[i])) {
      value.push(jewel[x].second);
      ++x;
    }
    
    if(!value.empty()) {
      int top = value.top(); value.pop();
      ans += top;
    }
  }
}