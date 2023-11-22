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
    int n;
    vector<pll> points;
} Input;

typedef struct Output {
    double area;
} Output;

Input input;
Output output;

void fn_input() {
    cin >> input.n;
    for (int i = 0; i < input.n; ++i) {
        ll x, y; cin >> x >> y;
        input.points.emplace_back(x, y);
    }
}

void fn_output() {
    cout << fixed;
    cout.precision(1);

    cout << abs(output.area) << endl;
}

ll ccw(pll a, pll b, pll c) {
    ll x = a.first * b.second + b.first * c.second + c.first * a.second;
    ll y = a.second * b.first + b.second * c.first + c.second * a.first;

    return x - y;
}

void fn_solve() {
    for (int i = 1; i < input.n - 1; ++i) {
        ll area = ccw(input.points[0], input.points[i], input.points[i + 1]);
        output.area += (double)area / 2;
    }
}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}