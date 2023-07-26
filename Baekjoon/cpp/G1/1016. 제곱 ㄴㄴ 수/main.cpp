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

ll _min, _max, ans;
bool check[1000001];

int main () {
    sync();
    cin >> _min >> _max;

    for(ll i = 2; i * i <= _max; ++i) {
        ll remainder = _min % (i * i) ? (i * i) - _min % (i * i) : 0;

        for(ll j = remainder; j <= (_max - _min + 1); j += (i * i)) { 
            check[j] = true;
        }
    }

    for(int i = 0; i < (_max - _min + 1); ++i) {
        if(check[i] == false)
            ++ans;
    }

    cout << ans << endl;
    return 0;
}