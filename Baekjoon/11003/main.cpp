#include <bits/stdc++.h>
#define sync() ios_base::sync_with_stdio(0); cin.tie(0)
#define endl "\n"
#define ends " "
#define fs first
#define se second
#define pb push_back
#define all(x) (x).begin, (x).end()
#define getx(x, i) get<(x)>(i)
using namespace std;

typedef long long ll;
typedef pair<int ,int> pii;
typedef pair<ll, ll> pll;
typedef vector<int>  vi;
typedef vector<ll> vl;
typedef vector<bool> vb;
typedef vector<pii> vii;
typedef vector<pll> vll;
typedef tuple<int, int, int> ti;
typedef tuple<ll, ll, ll> tl;

const int INF = 0x3f3f3f3f;
const ll LINF = 0x3f3f3f3f3f3f3f3f;

const int dx[] = {-1, 0, 1, 0};
const int dy[] = {0, -1, 0, 1};

int N, L;
vi A;

int main() {
  sync(); cin >> N >> L;
  A.assign(N, 0);

  for(int i = 0; i < N; ++i) cin >> A[i];

  priority_queue<pii> pq;
  for (int i = 0; i < N; ++i) {
    pq.push({ -A[i], i });
    
    while(pq.top().se < i - L + 1) pq.pop();

    cout << -pq.top().fs << ends;
  }

  return 0;
}

/*
예제 입력:
12 3
1 5 2 3 6 2 3 7 3 5 2 6

예제 출력:
1 1 1 2 2 2 2 2 3 3 2 2
*/