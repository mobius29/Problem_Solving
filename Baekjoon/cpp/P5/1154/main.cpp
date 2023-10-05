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
typedef vector<bool> vb;
typedef vector<pii> vii;
typedef vector<pll> vll;
typedef tuple<int, int, int> ti;
typedef tuple<ll, ll, ll> tl;

const int INF = 0x3f3f3f3f;
const ll LINF = 0x3f3f3f3f3f3f3f3f;

const int dx[] = {-1, 0, 1, 0};
const int dy[] = {0, -1, 0, 1};

int N;
set<int> team_1, team_2;
vector<vb> adjacent_matrix;

bool isCompletedGraph(set<int> graph){
	set<int>::iterator iter, iter2;
	for(iter = graph.begin(); iter != graph.end(); ++iter) {
        for(iter2 = graph.begin(); iter2 != graph.end(); ++iter2) {
            if (iter == iter2) continue;
            if (!adjacent_matrix[*iter][*iter2]) return false;
        }	
	}

    return true;
}

bool solve() {
    while(true) {
        if (!isCompletedGraph(team_2)) return false;
        if (isCompletedGraph(team_1)) return true;

        set<int>::iterator iter, iter2;
        int maxUnlinked = -1, maxUnlinkedNode = -1;
        for (iter = team_1.begin(); iter != team_1.end(); ++iter) {
            int unLinked = 0;
            for (iter2 = team_1.begin(); iter2 != team_1.end(); ++iter2) {
                if (!adjacent_matrix[*iter][*iter2]) unLinked++;
            }

            if (unLinked > maxUnlinked) {
                maxUnlinked = unLinked;
                maxUnlinkedNode = *iter;
            }
        }

        team_1.erase(maxUnlinkedNode);
        team_2.insert(maxUnlinkedNode);
    }
}

int main () {
    sync(); cin >> N;
    adjacent_matrix.assign(N+1, vb(N+1, false));

    while(true) {
        int a, b; cin >> a >> b;
        if (a == -1 && b == -1) break;

        adjacent_matrix[a][b] = adjacent_matrix[b][a] = true;
    }

    team_1.insert(1);
    for (int i = 2; i <= N; ++i) {
        if (adjacent_matrix[1][i]) team_1.insert(i);
        else team_2.insert(i);
    }

    bool result = solve();
    if (!result) cout << -1 << endl;
    else {
        cout << 1 << endl;

        set<int>::iterator iter;
        for (iter = team_1.begin(); iter != team_1.end(); ++iter) {
            cout << *iter << ends;
        }
        cout << -1 << endl;

        for (iter = team_2.begin(); iter != team_2.end(); ++iter) {
            cout << *iter << ends;
        }
        cout << -1 << endl;
    }

    return 0;
}
