#include <cmath>
#include <iomanip>
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

/**
## 문제
평면 상에 N개의 점이 찍혀있고, 그 점을 집합 P라고 하자. 집합 P의 벡터 매칭은
벡터의 집합인데, 모든 벡터는 집합 P의 한 점에서 시작해서, 또 다른 점에서 끝나는
벡터의 집합이다. 또, P에 속하는 모든 점은 한 번씩 쓰여야 한다.

벡터 매칭에 있는 벡터의 개수는 P에 있는 점의 절반이다.

평면 상의 점이 주어졌을 때, 집합 P의 벡터 매칭에 있는 벡터의 합의 길이의
최솟값을 출력하는 프로그램을 작성하시오.

## 입력
첫째 줄에 테스트 케이스의 개수 T가 주어진다. 각 테스트 케이스는 다음과 같이
구성되어있다.

테스트 케이스의 첫째 줄에 점의 개수 N이 주어진다. N은 짝수이다. 둘째 줄부터
N개의 줄에 점의 좌표가 주어진다. N은 20보다 작거나 같은 자연수이고, 좌표는
절댓값이 100,000보다 작거나 같은 정수다. 모든 점은 서로 다르다.

## 출력
각 테스트 케이스마다 정답을 출력한다. 절대/상대 오차는 10-6까지 허용한다.
*/

int T, N;
ll answer;

vii points;
pll point_sum;

void solve(vector<pii> &v, int n) {
  if (v.size() == (N / 2)) {
    pll selected_sum = {0, 0};
    for (auto [vx, vy] : v) {
      selected_sum.first += vx;
      selected_sum.second += vy;
    }

    pll not_selected_sum;
    not_selected_sum.first = point_sum.first - selected_sum.first;
    not_selected_sum.second = point_sum.second - selected_sum.second;

    ll x_square = pow(selected_sum.first - not_selected_sum.first, 2);
    ll y_square = pow(selected_sum.second - not_selected_sum.second, 2);

    ll dist = x_square + y_square;
    answer = min(answer, dist);
    return;
  }

  for (int i = n + 1; i < N; i++) {
    v.push_back(points[i]);
    solve(v, i);
    v.pop_back();
  }
  return;
}

int main() {
  sync();
  cin >> T;

  while (T--) {
    answer = LINF;
    points.clear();
    point_sum = {0, 0};

    cin >> N;
    for (int i = 0; i < N; i++) {
      int x, y;
      cin >> x >> y;

      point_sum.first += x;
      point_sum.second += y;

      points.push_back({x, y});
    }

    vector<pii> v;
    solve(v, 0);

    cout << setprecision(15) << sqrt(answer) << endl;
  }

  return 0;
}
