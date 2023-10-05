#include <bits/stdc++.h>
#define sync() ios_base::sync_with_stdio(0); cin.tie(0)
#define endl "\n"
#define ends " "
#define fs first
#define se second
#define pb push_back
#define all(x) (x).begin(), (x).end()
#define getx(x, i) get<(x)>(i)
#define MAX 40001
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

int N, M, max_depth;
int dist_from_first[MAX], depth[MAX];
vi parent[MAX];
vii dist[MAX];

void get_distance_and_parent_tree() {
    queue<int> q;
    bool visited[MAX] = { false, }; visited[1] = true;
    dist_from_first[1] = 0; q.push(1);

    max_depth = (int)floor(log2(MAX));
    for(int i = 0; i < MAX; ++i)
        parent[i].assign(max_depth+1, 0);

    while(!q.empty()) {
        int cur_node = q.front(); q.pop();

        for (auto [next_node, next_dist]: dist[cur_node]) {
            if (visited[next_node]) continue;
            visited[next_node] = true;

            dist_from_first[next_node] = dist_from_first[cur_node] + next_dist;
            depth[next_node] = depth[cur_node] + 1;

            parent[next_node][0] = cur_node;
            for (int i = 1; i <= max_depth; ++i) {
                int parent_node = parent[next_node][i-1];
                parent[next_node][i] = parent[parent_node][i-1];
            }

            q.push(next_node);
        }
    }
}

int main() {
    sync(); cin >> N;
    for (int i = 0; i < N-1; ++i) {
        int a, b, d; cin >> a >> b >> d;
        dist[a].pb({ b, d });
        dist[b].pb({ a, d });
    }

    get_distance_and_parent_tree();

    cin >> M;
    for (int i = 0; i < M; ++i) {
        int a, b; cin >> a >> b;

        if (depth[a] < depth[b]) swap(a, b);
        int x = a, y = b;

        while(depth[a] != depth[b]) {
            for (int i = max_depth; i >= 0; --i) {
                int parent_node = parent[a][i];
                if (depth[parent_node] >= depth[b]) {
                    a = parent_node;
                    break;
                }
            }
        }

        while(a != b) {
            for (int i = 0; i <= max_depth; ++i) {
                int parent_a = parent[a][i], parent_b = parent[b][i];

                if (parent_a == parent_b) break;
                a = parent_a, b = parent_b;
            }

            a = parent[a][0], b = parent[b][0];
        }
        
        cout << dist_from_first[x] + dist_from_first[y] - dist_from_first[a] * 2 << endl;
    }
}