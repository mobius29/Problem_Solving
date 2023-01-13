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

int main () {
  sync();
  int N; cin >> N;
  vi arr(N), LIS;
  vii idx(N);
  for (int i = 0; i < N; ++i)
    cin >> arr[i];

  LIS.pb(arr[0]); idx[0] = { 0, arr[0] };
  for (int i = 1; i < N; ++i) {
    if (arr[i] > LIS.back()) {
      LIS.pb(arr[i]);
      idx[i] = { LIS.size() - 1, arr[i] };
      continue;
    }

    vi::iterator low = lower_bound(all(LIS), arr[i]);
    int index = low - LIS.begin();
    LIS[index] = arr[i];
    idx[i] = { index, arr[i] };
  }

  vi ans;
  int num = LIS.size() - 1;
  for (int i = N-1; i >= 0; --i) {
    if (idx[i].first == num) {
      ans.pb(idx[i].second);
      --num;
    }
  }

  cout << LIS.size() << endl;
  for (int i = ans.size() - 1; i >= 0; --i)
    cout << ans[i] << ends;
  cout << endl;

  return 0;
}