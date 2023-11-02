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
    vector<vector<char>> board;
} Input;

typedef struct Output {
    int people{0};
} Output;

Input fn_input() {
    int N, M; cin >> N >> M;
    vector<vector<char>> board(N, vector<char>(M, 0));
    for (int i = 0; i < N; ++i)
        for (int j = 0; j < M; ++j)
            cin >> board[i][j];

    return { N, M, board };
}

void fn_output(Output &output) {
    if (output.people == 0) cout << "TT" << endl;
    else cout << output.people << endl;
}

pair<int, int> find_doyeon(Input &input) {
    for (int i = 0; i < input.N; ++i) {
        for (int j = 0; j < input.M; ++j) {
            if (input.board[i][j] == 'I') {
                return { i, j };
            }
        }
    }

    return { -1, -1 };
}

Output fn_solve(Input &input) {
    Output output;

    queue<pair<int, int>> q;
    vector<vector<bool>> is_visited(input.N, vector<bool>(input.M, false));

    pair<int, int> start = find_doyeon(input);
    q.emplace(start); is_visited[start.first][start.second] = true;

    while (!q.empty()) {
        auto [cur_x, cur_y] = q.front(); q.pop();

        for (auto [dx, dy]: d) {
            int next_x = cur_x + dx, next_y = cur_y + dy;

            if (next_x < 0 || next_x >= input.N) continue;
            if (next_y < 0 || next_y >= input.M) continue;
            if (is_visited[next_x][next_y]) continue;

            is_visited[next_x][next_y] = true;
            if (input.board[next_x][next_y] == 'X') continue;
            if (input.board[next_x][next_y] == 'P') {
                output.people += 1;
            }

            q.emplace(next_x, next_y);
        }
    }

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}

