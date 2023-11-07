#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
#include <set>
#include <stack>

#define endl '\n'
#define ends ' '

#define MOD 1000000007
using namespace std;

typedef long long ll;
typedef unsigned long long ull;

const int INF = 0x3F3F3F3F;

const pair<int, int> d[4] = { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 }};

typedef struct Input {
    string infix_expression;
} Input;

typedef struct Output {
    string postfix_expression;
} Output;

Input fn_input() {
    Input input;
    cin >> input.infix_expression;

    return input;
}

void fn_output(Output &output) {
    cout << output.postfix_expression << endl;
}

void make_postfix(stack<string> &operands, stack<char> &operators) {
    string second = operands.top(); operands.pop();
    string first = operands.top(); operands.pop();
    string oper = string(1, operators.top()); operators.pop();

    string new_string = first.append(second).append(oper);
    operands.push(new_string);
}

int get_operator_priority(char c) {
    if (c == '+' || c == '-') return 1;
    if (c == '*' || c == '/') return 2;

    return -1;
}

Output fn_solve(Input &input) {
    Output output;

    stack<string> operands;
    stack<char> operators;

    for (auto c: input.infix_expression) {
        if ('A' <= c && c <= 'Z') {
            operands.emplace(1, c);
            continue;
        }

        if (operators.empty() || c == '(') {
            operators.push(c);
            continue;
        }

        if (c == ')') {
            while (operators.top() != '(') make_postfix(operands, operators);
            operators.pop();

            continue;
        }

        int cur_priority = get_operator_priority(c);

        while (true) {
            if (operators.empty()) break;

            char top_operator = operators.top();
            int top_priority = get_operator_priority(top_operator);

            if (top_priority < cur_priority) break;

            make_postfix(operands, operators);
        }

        operators.push(c);
    }

    while (!operators.empty()) {
        make_postfix(operands, operators);
    }


    output.postfix_expression = operands.top(); operands.pop();

    return output;
}

int main() {
    Input input = fn_input();
    Output output = fn_solve(input);
    fn_output(output);

    return 0;
}