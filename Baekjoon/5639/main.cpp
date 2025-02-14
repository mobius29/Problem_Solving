#include <bits/stdc++.h>
#define sync() ios_base::sync_with_stdio(0); cin.tie(0)
#define endl "\n"
#define ends " "
#define fs first
#define se second
#define pb push_back
#define all(x) (x).begin(), (x).end()
#define getx(x, i) get<(x)>(i)
#define MAX 20000
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

vi number_list;

void pre_to_post_traverse(int l, int r) {
  if (l >= r) return;

  int m = l;
  while (number_list[++m] < number_list[l]);

  pre_to_post_traverse(l + 1, m);
  pre_to_post_traverse(m, r);

  cout << number_list[l] << endl;
}

int main() {
  sync();

  while (true) {
    int num; cin >> num;
    if (cin.eof()) break;

    number_list.pb(num);
  }
  number_list.pb(1e6);

  pre_to_post_traverse(0, number_list.size() - 1);
}