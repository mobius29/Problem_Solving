#include <iostream>
#include <queue>
#include <vector>

#define sync()                                                                 \
  ios_base::sync_with_stdio(0);                                                \
  cin.tie(0)
#define endl "\n"
#define ends " "
#define pb push_back
#define all(x) (x).begin(), (x).end()
#define getx(x, i) get<(x)>(i)
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;
typedef pair<ll, ll> pll;
typedef vector<int> vi;
typedef vector<ll> vl;
typedef vector<pii> vii;
typedef vector<pll> vll;
typedef tuple<int, int, int> ti;
typedef tuple<ll, ll, ll> tl;

const int INF = 0x3f3f3f3f;
const ll LINF = 0x3f3f3f3f3f3f3f3f;

const int dx[] = {-1, 0, 1, 0};
const int dy[] = {0, -1, 0, 1};

int N, answer;
vector<vii> LAN_cable;

int find_MST_value() {
  priority_queue<pii> pq;
  vector<bool> isVisited(N, false);

  int ret = 0;

  for (auto [next, next_v] : LAN_cable[0]) {
    pq.push({-next_v, next});
  }
  isVisited[0] = true;

  while (!pq.empty()) {
    auto [cur_v, cur] = pq.top();
    pq.pop();

    if (isVisited[cur])
      continue;
    isVisited[cur] = true;

    ret += (-cur_v);

    for (auto [next, next_v] : LAN_cable[cur]) {
      if (isVisited[next])
        continue;

      pq.push({-next_v, next});
    }
  }

  for (int i = 0; i < N; ++i) {
    if (!isVisited[i])
      return -1;
  }

  return ret;
}

int main() {
  sync();
  cin >> N;

  LAN_cable.assign(N, vii());

  int total_LAN = 0;

  for (int i = 0; i < N; ++i) {
    string str;
    cin >> str;

    for (int j = 0; j < N; ++j) {
      char c = str[j];

      if (c == '0')
        continue;

      int value = -1;

      if ('a' <= c && c <= 'z')
        value = c - 'a' + 1;

      if ('A' <= c && c <= 'Z')
        value = c - 'A' + 27;

      total_LAN += value;

      if (i == j)
        continue;

      LAN_cable[i].pb({j, value});
      LAN_cable[j].pb({i, value});
    }
  }

  int ret = find_MST_value();
  if (ret == -1) {
    cout << -1 << endl;
    return 0;
  }

  cout << total_LAN - find_MST_value() << endl;

  return 0;
}
