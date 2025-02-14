#include <iostream>
#include <vector>
#include <algorithm>
#include <stack>
#include <deque>

#define endl '\n'
#define ends ' '

using namespace std;

typedef pair<int, int> pii;
typedef long long ll;
typedef unsigned long long ull;

const int INF = 0x3F3F3F3F;

const pii d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    string str, bomb_str;
} Input;

typedef struct Output {
    string str;
} Output;

Input input;
Output output;

void fn_input() {
    cin >> input.str >> input.bomb_str;
}

void fn_output() {
    cout << output.str << endl;
}

void fn_solve() {
    deque<char> dq;

    for (char c: input.str) {
        dq.push_back(c);

        bool flag = true;
        deque<char> temp;
        for (int i = input.bomb_str.length() - 1; i >= 0; --i) {
            if (dq.empty() || dq.back() != input.bomb_str[i]) {
                flag = false;
                break;
            }

            temp.push_back(dq.back());
            dq.pop_back();
        }

        if (!flag) {
            while (!temp.empty()) {
                dq.push_back(temp.back());
                temp.pop_back();
            }
        }
    }


    while (!dq.empty()) {
        output.str.push_back(dq.front());
        dq.pop_front();
    }

    if (output.str.empty()) output.str = "FRULA";
}

int main() {
    fn_input();
    fn_solve();
    fn_output();

    return 0;
}