#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>

#define endl '\n'
#define ends ' '

using namespace std;


typedef long long ll;
typedef pair<int, int> pii;
typedef pair<ll, ll> pll;

const int INF = 0x3F3F3F3F;

const int dx[4] = { -1, 0, 1, 0 };
const int dy[4] = { 0, 1, 0, -1 };

typedef struct Input {
    int v, e;
    vector<vector<pii>> edges;
} Input;

typedef struct Output {
    int weight;
} Output;

Input input;
Output output;

void fn_input() {
    cin >> input.v >> input.e;

    input.edges.assign(input.v + 1, vector<pii>());
    for (int i = 0; i < input.e; ++i) {
        int a, b, c; cin >> a >> b >> c;
        input.edges[a].emplace_back(b, c);
        input.edges[b].emplace_back(a, c);
    }
}

void fn_output() {
    cout << output.weight << endl;
}

void fn_solve() {
    priority_queue<pii, vector<pii>, greater<>> pq;
    vector<bool> is_visited(input.v + 1, false);
    pq.emplace(0, 1); is_visited[1] = true;

    for (auto [next, next_dist]: input.edges[1]) {
        pq.emplace(next_dist, next);
    }

    while (!pq.empty()) {
        auto [dist, cur] = pq.top(); pq.pop();
        if (is_visited[cur]) continue;

        is_visited[cur] = true;
        output.weight += dist;

        for (auto [next, next_dist]: input.edges[cur]) {
            if (is_visited[next]) continue;

            pq.emplace(next_dist, next);
        }
    }


}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}