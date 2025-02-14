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
    int n, m;
    int src, dest;
    vector<vector<pii>> bus;
} Input;

typedef struct Output {
    int minimum_cost;
    vector<int> cities;
} Output;

Input input;
Output output;

void fn_input() {
    cin >> input.n >> input.m;

    input.bus.assign(input.n + 1, vector<pii>());
    for (int i = 0; i < input.m; ++i) {
        int a, b, c; cin >> a >> b >> c;
        input.bus[a].emplace_back(b, c);
    }

    cin >> input.src >> input.dest;
}

void fn_output() {
    cout << output.minimum_cost << endl;
    cout << output.cities.size() << endl;
    for (int city: output.cities) {
        cout << city << ends;
    }
}

void fn_solve() {
    vector<int> prev_city(input.n + 1, 0);
    for (int i = 1; i <= input.n; ++i) {
        prev_city[i] = i;
    }

    vector<int> distances(input.n + 1, INF);

    priority_queue<pii> pq; pq.emplace(0, input.src);
    distances[input.src] = 0;

    while (!pq.empty()) {
        auto [dist, cur_city] = pq.top(); pq.pop();
        dist = -dist;

        if (dist > distances[cur_city]) continue;

        for (auto [next, cost]: input.bus[cur_city]) {
            int next_dist = dist + cost;

            if (next_dist < distances[next]) {
                prev_city[next] = cur_city;
                distances[next] = next_dist;
                pq.emplace(-next_dist, next);
            }
        }
    }

    output.minimum_cost = distances[input.dest];

    int cur_city = input.dest;
    while (cur_city != input.src) {
        output.cities.push_back(cur_city);
        cur_city = prev_city[cur_city];
    }
    output.cities.push_back(cur_city);

    reverse(output.cities.begin(), output.cities.end());
}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}