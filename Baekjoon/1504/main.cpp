#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>

#define endl '\n'
#define ends ' '

using namespace std;

typedef pair<int, int> pii;
typedef long long ll;
typedef unsigned long long ull;

const int INF = 0x3F3F3F3F;

const pair<int, int> d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    int n{0}, e{0};
    int v1{0}, v2{0};
    vector<vector<pii>> paths;
} Input;

typedef struct Output {
    ll answer{INF};
} Output;

Input input;
Output output;

void fn_input() {
    cin >> input.n >> input.e;

    input.paths.assign(input.n + 1, vector<pii>());

    vector<vector<int>> v(input.n + 1, vector<int> (input.n + 1, INF));
    for (int i = 0; i < input.e; ++i) {
        int a, b, c; cin >> a >> b >> c;
        v[a][b] = min(v[a][b], c);
        v[b][a] = min(v[b][a], c);
    }

    for (int i = 1; i <= input.n; ++i) {
        for (int j = 1; j <= input.n; ++j) {
            if (v[i][j] < INF) {
                input.paths[i].emplace_back(j, v[i][j]);
            }
        }
    }

    cin >> input.v1 >> input.v2;
}

void fn_output() {
    if (output.answer == INF) output.answer = -1;
    cout << output.answer << endl;
}

void get_minimum_paths(int start, vector<vector<ll>> &minimum_paths) {
    priority_queue<pii> pq; pq.emplace( 0, start );
    while(!pq.empty()) {
        auto [dist, cur] = pq.top(); pq.pop();
        dist = -dist;

        for (auto [next, next_dist]: input.paths[cur]) {
            if (dist + next_dist < minimum_paths[start][next]) {
                minimum_paths[start][next] = dist + next_dist;
                pq.emplace(-minimum_paths[start][next], next);
            }
        }
    }

    minimum_paths[start][start] = 0;
}

void fn_solve() {
    vector<vector<ll>> minimum_paths(input.n + 1, vector<ll>(input.n + 1, INF));

    int v1 = input.v1, v2 = input.v2;
    get_minimum_paths(v1, minimum_paths);
    get_minimum_paths(v2, minimum_paths);

    ll v1_v2 = minimum_paths[v1][1] + minimum_paths[v1][v2] + minimum_paths[v2][input.n];
    ll v2_v1 = minimum_paths[v2][1] + minimum_paths[v2][v1] + minimum_paths[v1][input.n];

    ll minimum_path = min(v1_v2, v2_v1);

    output.answer = min(output.answer, minimum_path);
}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}