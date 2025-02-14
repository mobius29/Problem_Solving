#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>

#define endl '\n'
#define ends ' '

using namespace std;

typedef long long ll;
typedef unsigned long long ull;

const int INF = 0x3F3F3F3F;

const pair<int, int> d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    int n{0}, m{0};
    int src{0}, dest{0};
    vector<vector<pair<int, int>>> paths;
} Input;

typedef struct Output {
    int answer;
} Output;

Input fn_input() {
    Input input;

    cin >> input.n >> input.m;

    input.paths.assign(input.n + 1, vector<pair<int, int>>());
    for (int i = 0; i < input.m; ++i) {
        int start, end, cost; cin >> start >> end >> cost;
        input.paths[start].emplace_back(end, cost);
    }

    cin >> input.src >> input.dest;

    return input;
}

void fn_output(Output &output) {
    cout << output.answer << endl;
}

Output fn_solve(Input &input) {
    Output output;

    vector<int> dist_from_src(input.n + 1, INF);

    priority_queue<pair<int, int>> pq;
    pq.emplace(0, input.src);
    dist_from_src[input.src] = 0;

    while (!pq.empty()) {
        auto [cur_dist, cur_pos] = pq.top(); pq.pop();

        cur_dist = -cur_dist;
        if (cur_dist > dist_from_src[cur_pos]) continue;

        for (auto [next_pos, cost]: input.paths[cur_pos]) {
            int next_cost = cur_dist + cost;

            if (next_cost < dist_from_src[next_pos]) {
                dist_from_src[next_pos] = next_cost;
                pq.emplace(-next_cost, next_pos);
            }
        }
    }

    output.answer = dist_from_src[input.dest];

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}