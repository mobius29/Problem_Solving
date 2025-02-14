#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
#include <set>

#define endl '\n'
#define ends ' '

#define MOD 1000000007
using namespace std;

typedef long long ll;
typedef unsigned long long ull;

const int INF = 0x3F3F3F3F;

const pair<int, int> d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    ll n;
} Input;

typedef struct Output {
    ll answer;
} Output;

Input fn_input() {
    Input input;
    cin >> input.n;

    return input;
}

void fn_output(Output &output) {
    cout << output.answer << endl;
}

vector<vector<ll>> multiply_matrix(const vector<vector<ll>> &v1, const vector<vector<ll>> &v2) {
    vector<vector<ll>> ret(2, vector<ll> (2, 0));

    for (int i = 0; i < 2; ++i) {
        for (int j = 0; j < 2; ++j) {
            for (int k = 0; k < 2; ++k) {
                ret[i][j] += (v1[i][k] * v2[k][j]) % MOD;
                ret[i][j] %= MOD;
            }
        }
    }

    return ret;
}

vector<vector<ll>> initial_matrix;

vector<vector<ll>> power_matrix(ll n) {
    if (n == 1) {
        vector<vector<ll>> ret(2, vector<ll>(2, 1));
        ret[1][1] = 0;

        return ret;
    }

    vector<vector<ll>> n_division_2_powered_matrix = power_matrix(n / 2);
    vector<vector<ll>> squared_matrix = multiply_matrix(n_division_2_powered_matrix, n_division_2_powered_matrix);

    if (n % 2 == 0) {
        return squared_matrix;
    }

    else {
        return multiply_matrix(squared_matrix, initial_matrix);
    }
}

Output fn_solve(Input &input) {
    Output output;

    initial_matrix.assign(2, vector<ll> (2, 1));
    initial_matrix[1][1] = 0;

    vector<vector<ll>> powered_matrix = power_matrix(input.n);
    output.answer = powered_matrix[0][1];

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}

