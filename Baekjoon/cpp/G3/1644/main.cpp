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

int n, answer;

void input() {
  cin >> n;
}

void output() {
  cout << answer << endl;
}

void solve() {
  vector<bool> is_prime(n + 1, true);
  vector<int> primes;

  for (int i = 2; i <= n; ++i) {
    if (is_prime[i]) {
      primes.emplace_back(i);

      for (int j = i * 2; j <= n; j += i) {
        is_prime[j] = false;
      }
    }
  }

  int left = 0, right = 0;
  int current_sum = 0;

  while (left <= right && left < primes.size() && right <= primes.size()) {
    if (current_sum == n) {
      ++answer;
      current_sum += primes[right++];
      current_sum -= primes[left++];

      continue;
    }

    if (current_sum < n) {
      current_sum += primes[right++];
    } else {
      current_sum -= primes[left++];
    }
  }
}

int main() {
  sync();
  input();
  solve();
  output();
}