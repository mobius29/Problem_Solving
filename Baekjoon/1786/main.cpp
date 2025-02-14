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

vi failFunction(string pattern) {
  vi table(pattern.length(), 0);

  for(int i = 1, j = 0; i < pattern.length(); ++i){
    while (j > 0 && pattern[i] != pattern[j]) j = table[j-1];

    if (pattern[i] == pattern[j])
      table[i] = ++j;
  }

  return table;
}

vi KMP(string str, string pattern) {
  vi table = failFunction(pattern);
  vi ret;

  for(int i = 0, j = 0; i < str.length(); ++i) {
    while (j > 0 && str[i] != pattern[j]) j = table[j-1];
    
    if (str[i] == pattern[j]) {
      if (j == pattern.length() - 1) {
        ret.pb(i-j);
        i -= (j - table[j-1]);
        j = table[j-1];
      }

      else ++j;
    }
  }

  return ret;
}

int main () {
  sync();
  string T, P;
  getline(cin, T); getline(cin, P);

  vi res = KMP(T, P);

  cout << res.size() << endl;
  for (int i = 0; i < res.size(); ++i)
    cout << res[i] + 1 << ends;

  return 0;
}