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

int n;
vector<int> student;

void init() {
    cin >> n;

    student.clear(); student.assign(n + 1, 0);

    for(int i = 1; i <= n; ++i){
        cin >> student[i];
    }
}


int solve();
int dfs(int, vector<bool>&, vector<int>&);

int main () {
    sync();
    int T; cin >> T;

    while(T--) {
        init();

        cout << solve() << endl;
    }
    

    return 0;
}

int solve() {
    int ans = 0;
    vector<int> v;
    vector<bool> isVisited(n + 1, false);

    for(int i = 1; i <= n; ++i) {
        if(isVisited[i]) continue;

        int ret = dfs(i, isVisited, v);
        
        int add = 0;
        while(!v.empty()) {
            int back = v.back(); v.pop_back();

            add = back == ret ? 0 : ++add;
        }

        ans += add;
    }

    return ans;
}

int dfs(int cur, vector<bool>& isVisited, vector<int>& v) {
    if(isVisited[cur]) return cur;

    isVisited[cur] = true;
    v.pb(cur);

    return dfs(student[cur], isVisited, v);
}