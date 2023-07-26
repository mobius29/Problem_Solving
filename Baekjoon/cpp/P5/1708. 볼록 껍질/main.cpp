#include <bits/stdc++.h>
#define sync() ios_base::sync_with_stdio(0); cin.tie(0)
#define endl "\n"
#define ends " "
#define fs first
#define se second
#define pb push_back
#define all(x) (x).begin(), (x).end()
#define getx(x, i) get<(x)>(i)
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;
typedef pair<ll, ll> pll;
typedef vector<int>  vi;
typedef vector<ll> vl;
typedef vector<bool> vb;
typedef vector<pll> vii;
typedef vector<pll> vll;
typedef tuple<int, int, int> ti;
typedef tuple<ll, ll, ll> tl;
typedef pll point;

const int INF = 0x3f3f3f3f;
const ll LINF = 0x3f3f3f3f3f3f3f3f;

const int dx[] = {-1, 0, 1, 0};
const int dy[] = {0, -1, 0, 1};

int N;
pll min_y_point;
vll point_list;

ll get_ccw(pll, pll, pll);
ll get_distance(pll, pll);
bool compare(pll, pll);
stack<pll> solve();

int main() {
  sync(); cin >> N;
  for(int i = 0; i < N; ++i) {
    ll x, y; cin >> x >> y;
    point_list.pb({ x, y });
  }

  min_y_point = point_list[0];
  for (int i = 1; i < N; ++i) {
    pll item = point_list[i];
    if (item.second > min_y_point.second) continue;

    if (item.second < min_y_point.second || item.first < min_y_point.first) {
      min_y_point = item;
    }
  }

  sort(all(point_list), compare);
  
  stack<pll> answer = solve();

  cout << answer.size() << endl;
}

ll get_ccw(pll a, pll b, pll c) {
  return (b.first - a.first) * (c.second - a.second) - (c.first - a.first) * (b.second - a.second);
}

ll get_distance(pll a, pll b) {
  ll x = b.first - a.first;
  ll y = b.second - a.second;

  return x * x + y * y;
}

bool compare(pll a, pll b) {
  ll ccw = get_ccw(min_y_point, a, b);

  if (ccw == 0) {
    return get_distance(min_y_point, a) < get_distance(min_y_point, b);
  }

  return ccw > 0;
}

stack<pll> solve() {
  stack<pll> st;
  st.push(point_list[0]); st.push(point_list[1]);

  for (int i = 2; i < N; ++i) {
    st.push(point_list[i]);
    while (st.size() > 2) {
      pll c = st.top(); st.pop();
      pll b = st.top(); st.pop();
      pll a = st.top();

      ll ccw = get_ccw(a, b, c);

      if (ccw > 0) {
        st.push(b); st.push(c);
        break;
      }

      st.push(c);
      if (ccw == 0) break;
    }
  }

  return st;
}