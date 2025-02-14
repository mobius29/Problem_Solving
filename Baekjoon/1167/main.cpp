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

const pii d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    int n{0};
    vector<vector<pii>> tree;
} Input;

typedef struct Output {
    int answer;
} Output;

Input fn_input() {
    Input input;

    cin >> input.n;

    input.tree.assign(input.n + 1, vector<pii> ());
    for (int i = 0; i < input.n; ++i) {
        int node; cin >> node;
        while (true) {
            int a, b; cin >> a;
            if (a == -1) break;
            cin >> b;

            input.tree[node].emplace_back(a, b);
        }
    }

    return input;
}

void fn_output(Output &output) {
    cout << output.answer << endl;
}

pii dfs(const vector<vector<pii>> &graph, vector<bool> &is_visited, int cur) {
    int ret_node = cur, ret_distance = 0;
    for (auto [next_node, next_distance]: graph[cur]) {
        if (is_visited[next_node]) continue;
        is_visited[next_node] = true;

        auto [node, distance] = dfs(graph, is_visited, next_node);
        int cur_dist = distance + next_distance;

        if (ret_distance < cur_dist) {
            ret_distance = cur_dist;
            ret_node = node;
        }
    }

    return { ret_node, ret_distance };
}

Output fn_solve(Input &input) {
    Output output;

    vector<bool> is_visited(input.n + 1, false); is_visited[1] = true;
    auto [node, dist] = dfs(input.tree, is_visited, 1);

    is_visited.assign(input.n + 1, false); is_visited[node] = true;
    auto [max_node, max_dist] = dfs(input.tree, is_visited, node);

    output.answer = max_dist;
    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}