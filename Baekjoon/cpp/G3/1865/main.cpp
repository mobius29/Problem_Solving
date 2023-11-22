#include <iostream>
#include <vector>
#include <algorithm>
#include <stack>


#define endl '\n'
#define ends ' '

using namespace std;

typedef pair<int, int> pii;
typedef long long ll;
typedef unsigned long long ull;

const int INF = 0x3F3F3F3F;

const pii d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    int n, m, w;
    vector<vector<pii>> dists;
} Input;

typedef struct Output {
    bool is_possible;
} Output;

int T;
Input input;
Output output;

void fn_input() {
    cin >> input.n >> input.m >> input.w;

    vector<vector<int>> paths(input.n + 1, vector<int> (input.n + 1, INF));
    for (int i = 0; i < input.m; ++i) {
        int s, e, t; cin >> s >> e >> t;
        paths[s][e] = paths[e][s] = min(paths[s][e], t);
    }

    for (int i = 0; i < input.w; ++i) {
        int s, e, t; cin >> s >> e >> t;
        paths[s][e] = min(paths[s][e], -t);
    }

    input.dists.assign(input.n + 1, vector<pii>());
    for (int i = 1; i <= input.n; ++i)
        for (int j = 1; j <= input.n; ++j)
            if (paths[i][j] != INF) input.dists[i].emplace_back(j, paths[i][j]);
}

void fn_output() {
    cout << (output.is_possible ? "YES" : "NO") << endl;
}

bool Bellman_Ford_Algorithm(int start, vector<bool> &is_visited) {
    vector<int> dist_from_start(input.n + 1, INF);
    dist_from_start[start] = 0;

    for (int i = 0; i < input.n - 1; ++i) {
        if (i == start) continue;
        for (int from = 1; from <= input.n; ++from) {
            for (auto [to, cost]: input.dists[from]) {
                if (dist_from_start[from] == INF) continue;
                dist_from_start[to] = min(dist_from_start[to], dist_from_start[from] + cost);
            }
        }
    }

    for (int from = 1; from <= input.n; ++from) {
        for (auto [to, cost]: input.dists[from]) {
            if (dist_from_start[from] == INF) continue;

            if (dist_from_start[to] != INF) is_visited[to] = true;

            if (dist_from_start[from] + cost < dist_from_start[to])
                return true;
        }
    }

    return false;
}


void fn_solve() {
    vector<bool> is_visited(input.n + 1, false);
    output.is_possible = false;

    for (int i = 1; i <= input.n; ++i) {
        if (is_visited[i]) continue;
        bool has_negative_cycle = Bellman_Ford_Algorithm(i, is_visited);

        if (has_negative_cycle) {
            output.is_possible = true;
            break;
        }
    }

}

int main() {
    cin >> T;

    while (T--) {
        fn_input();
        fn_solve();
        fn_output();
    }

    return 0;
}