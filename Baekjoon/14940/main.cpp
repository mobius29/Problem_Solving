#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>

#define endl '\n'
#define ends ' '
using namespace std;

typedef pair<int, int> pii;

const int INF = 0x3F3F3F3F;

const pair<int, int> d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    int N{0};
    int M{0};
    vector<vector<int>> board;
} Input;

typedef struct Output {
    int N{0};
    int M{0};
    vector<vector<int>> board;
} Output;

Input fn_input() {
    int N, M; cin >> N >> M;
    vector<vector<int>> board(N, vector<int>(M, 0));
    for (int i = 0; i < N; ++i)
        for (int j = 0; j < M; ++j)
            cin >> board[i][j];

    return { N, M, board };
}

void fn_output(Output output) {
    for (int i = 0; i < output.N; ++i) {
        for (int j = 0; j < output.M; ++j) {
            cout << output.board[i][j] << ends;
        }
        cout << endl;
    }
}

pair<int, int> find_destination_position(const Input &input) {
    for (int i = 0; i < input.N; ++i) {
        for (int j = 0; j < input.M; ++j) {
            if (input.board[i][j] == 2) {
                return { i, j };
            }
        }
    }

    return { -1, -1 };
}

typedef struct Node {
    pair<int, int> position;
    int count;
} Node;

vector<vector<int>> find_minimum_paths(Input &input) {
    auto destination = find_destination_position(input);

    vector<vector<int>> ret(input.N, vector<int>(input.M, 0));
    queue<Node> q; q.push({ destination, 0 });

    vector<vector<bool>> is_visited(input.N, vector<bool> (input.M, false));
    is_visited[destination.first][destination.second] = true;

    while (!q.empty()) {
        auto [cur_position, cur_count] = q.front(); q.pop();
        auto [cur_x, cur_y] = cur_position;
        int next_count = cur_count + 1;

        for (auto [dx, dy]: d) {
            int next_x = cur_x + dx, next_y = cur_y + dy;

            if (next_x < 0 || next_x >= input.N) continue;
            if (next_y < 0 || next_y >= input.M) continue;
            if (is_visited[next_x][next_y]) continue;
            is_visited[next_x][next_y]= true;

            if (input.board[next_x][next_y] != 1) continue;
            input.board[next_x][next_y] = 2;

            ret[next_x][next_y] = next_count;
            q.push({{ next_x, next_y }, next_count });
        }
    }

    return ret;
}

Output fn_solve(Input &input) {
    vector<vector<int>> ret = find_minimum_paths(input);

    for (int i = 0; i < input.N; ++i) {
        for (int j = 0; j < input.M; ++j) {
            if (input.board[i][j] == 1) {
                ret[i][j] = -1;
            }
        }
    }

    return { input.N, input.M, ret };
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);
    return 0;
}

