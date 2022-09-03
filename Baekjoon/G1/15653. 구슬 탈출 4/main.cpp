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

int N, M, ans = -1;
char board[10][10];
bool check[10][10][10][10];
pii red, blue, goal;

pii _move(pii, pii, int);
void _bfs();
void solve();

int main () {
    sync();
    cin >> N >> M;
    
    for(int i = 0; i < N; ++i) {
      for(int j = 0; j < M; ++j) {
        cin >> board[i][j];

        if(board[i][j] == 'R') red = {i, j};
        if(board[i][j] == 'B') blue = {i, j};
        if(board[i][j] == 'O') goal = {i, j};
      }
    }

    auto [rx, ry] = red;
    auto [bx, by] = blue;
    check[rx][ry][bx][by] = true;

    solve();

    cout << ans << endl;

    return 0;
}

pii _move(pii move_ball, pii fixed_ball, int dir) {
  pii pos = move_ball;

  while(true) {
    if(pos == goal) return pos;

    if(board[pos.first][pos.second] == '#' || pos == fixed_ball)
      return {pos.first - dx[dir], pos.second - dy[dir]};

    int nx = pos.first + dx[dir], ny = pos.second + dy[dir];
    pos = {nx, ny};
  }

  return {-1, -1};
}

void _bfs() {
  queue<pii> red_position;
  queue<pii> blue_position;
  queue<int> count;

  red_position.push(red);
  blue_position.push(blue);
  count.push(0);

  while(!count.empty()) {
    pii red_ball = red_position.front();
    red_position.pop();
    pii blue_ball = blue_position.front();
    blue_position.pop();
    int cnt = count.front();
    count.pop();

    if(blue_ball == goal) continue;

    if(red_ball == goal) {
      ans = cnt;
      break;
    }

    for(int i = 0; i < 4; ++i) {
      pii red_ball_move1 = _move(red_ball, blue_ball, i);
      pii blue_ball_move = _move(blue_ball, red_ball_move1, i);
      pii red_ball_move = _move(red_ball_move1, blue_ball_move, i);

      auto [nrx, nry] = red_ball_move;
      auto [brx, bry] = blue_ball_move;

      if(red_ball == red_ball_move && blue_ball == blue_ball_move) continue;

      if(check[nrx][nry][brx][bry]) continue;
      check[nrx][nry][brx][bry] = true;

      red_position.push(red_ball_move);
      blue_position.push(blue_ball_move);
      count.push(cnt + 1);
    }
  }
}

void solve() {
  _bfs();
}