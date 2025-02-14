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
    int n { 0 };
    vector<vector<pii>> paths;
} Input;

typedef struct Output {
    int answer { INF };
} Output;

Input input;
Output output;

void fn_input() {
    cin >> input.n;

    input.paths.assign(input.n + 1, vector<pii>());
    for (int i = 0; i < input.n - 1; ++i) {
        int a, b, c; cin >> a >> b >> c;
        input.paths[a].emplace_back(b, c);
        input.paths[b].emplace_back(a, c);
    }
}

void fn_output() {
    cout << output.answer << endl;
}

pii fn_get_max_distance_from_start(int cur, vector<bool> &is_visited) {
    pii ret = { 0, cur };

    for (auto [next_node, distance]: input.paths[cur]) {
        if (is_visited[next_node]) continue;

        is_visited[next_node] = true;
        auto [get_distance, get_node] = fn_get_max_distance_from_start(next_node, is_visited);

        if (ret.first < get_distance + distance) {
            ret.first = get_distance + distance;
            ret.second = get_node;
        }
    }

    return ret;
}


void fn_solve() {
    vector<bool> is_visited(input.n + 1, false); is_visited[1] = true;
    auto [sub_distance, sub_node] = fn_get_max_distance_from_start(1, is_visited);

    is_visited.assign(input.n + 1, false); is_visited[sub_node] = true;
    auto [max_distance, max_node] = fn_get_max_distance_from_start(sub_node, is_visited);

    output.answer = max_distance;
}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}