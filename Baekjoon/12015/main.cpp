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
    vi arr(N);
    for(int i = 0; i < N; ++i)
      cin >> arr[i];

    vi LIS; LIS.pb(arr[0]);
    for (int i = 1; i < N; ++i) {
      if (arr[i] > LIS.back()) {
        LIS.pb(arr[i]);
        continue;
      }

      vi::iterator low = lower_bound(all(LIS), arr[i]);
      int idx = low - LIS.begin();

      LIS[idx] = arr[i];
    }

    cout << LIS.size() << endl;

    return 0;
}