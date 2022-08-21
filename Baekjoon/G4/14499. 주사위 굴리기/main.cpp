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

const int dx[] = {0, 0, -1, 1};
const int dy[] = {1, -1, 0, 0};

int N, M, x, y, K;
pii pos;
int up = 1, east = 3, north = 2;
int board[20][20];
int dice[7]; // 0, 1, 2, 3, 4, 5, 6

void solve(int);
void rotate(int);

int main () {
    sync(); cin >> N >> M >> x >> y >> K;

    for(int i = 0; i < N; ++i) {
      for(int j = 0; j < M; ++j) {
        cin >> board[i][j];
      }
    }

    pos = {x, y};

    for(int i = 0; i < K; ++i) {
      int dir; cin >> dir;
      solve(dir - 1);
    }

    return 0;
}

void rotate(int dir) {
  int nUp, nEast, nNorth;

  if (dir == 0) {
    nUp = 7 - east;
    nEast = up;
    nNorth = north;
  }

  else if (dir == 1) {
    nUp = east;
    nEast = 7 - up;
    nNorth = north;
  }

  else if (dir == 2) {
    nUp = 7 - north;
    nEast = east;
    nNorth = up;
  }

  else {
    nUp = north;
    nEast = east;
    nNorth = 7 - up;
  }

  up = nUp, east = nEast, north = nNorth;
}

void solve(int dir) {
  auto [x, y] = pos;
  int nx = x + dx[dir], ny = y + dy[dir];

  if (nx < 0 || ny < 0 || nx >= N || ny >= N) return;
  pos = {nx, ny};

  rotate(dir);
  cout << dice[up] << endl;

  int down = 7 - up;

  if(!board[nx][ny]) {
    board[nx][ny] = dice[down];
  }

  else {
    dice[down] = board[nx][ny];
    board[nx][ny] = 0;
  }
}
