#include <cmath>
#include <iostream>
#include <vector>
#define sync()                                                                 \
  ios_base::sync_with_stdio(0);                                                \
  cin.tie(0)
#define endl "\n"
#define ends " "
#define all(x) (x).begin(), (x).end()
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;
typedef pair<ll, ll> pll;
typedef vector<int> vi;
typedef vector<ll> vl;
typedef vector<pii> vii;
typedef vector<pll> vll;

const int INF = 0x3f3f3f3f;
const ll LINF = 0x3f3f3f3f3f3f3f3f;

const int dx[] = {-1, 0, 1, 0};
const int dy[] = {0, -1, 0, 1};

int N, K, R1, R2, C1, C2;
vector<vector<bool>> board; // true - black, false - white

void paintBlack(const int r, const int c, const int size) {
  for (int i = r; i < r + size; ++i) {
    for (int j = c; j < c + size; ++j) {
      board[i][j] = true;
    }
  }
}

void solve(const int s, const int r, const int c) {
  if (s == 0)
    return;

  const int splitted_side_length = pow(N, (s - 1));

  const int ms_idx = (N - K) / 2;
  const int me_idx = (N + K) / 2;

  for (int i = 0; i < N; ++i) {
    int n_rs = r + splitted_side_length * i;
    int n_re = n_rs + splitted_side_length;
    if (n_rs < R1 || n_re > R2)
      continue;

    for (int j = 0; j < N; ++j) {
      int n_cs = c + splitted_side_length * j;
      int n_ce = n_cs + splitted_side_length;

      if (n_cs < C1 || n_ce > C2)
        continue;

      bool is_black_area = false;

      if (i >= ms_idx && i <= me_idx && j >= ms_idx && j <= me_idx)
        is_black_area = true;

      if (is_black_area) {
        paintBlack(n_rs, n_cs, splitted_side_length);
        continue;
      }

      solve(s - 1, n_rs, n_cs);
    }
  }
}

int main() {
  sync();

  int s = -1;
  cin >> s >> N >> K >> R1 >> R2 >> C1 >> C2;

  const int size = pow(N, s);
  board.assign(size, vector<bool>(size, false));

  solve(s, 0, 0);

  for (int i = R1; i <= R2; ++i) {
    for (int j = C1; j <= C2; ++j) {
      cout << (board[i][j] ? '1' : '0');
    }
    cout << endl;
  }

  return 0;
}
