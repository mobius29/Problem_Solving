#include <bits/stdc++.h>
#define sync() ios_base::sync_with_stdio(0); cin.tie(0)
#define endl "\n"
#define ends " "
#define MAX 1000000
#define NODE_MAX 3000000
using namespace std;

int N, seg_tree[NODE_MAX];

int whenAisOne(int);
void whenAisTwo(int, int);

int main() {
  sync(); cin >> N;

  while(N--) {
    int A; cin >> A;

    if (A == 1) {
        int B; cin >> B;
        cout << whenAisOne(B) << endl;
    }
    if (A == 2) {
        int B, C; cin >> B >> C;
        whenAisTwo(B, C);
    }
  }
}

int whenAisOne(int B) {
    int cur = 1;
    int start = 1, end = MAX;

    while(start < end) {
        seg_tree[cur] -= 1;
        int m = (start + end) / 2;

        int next_node = cur * 2;
        if (seg_tree[next_node] >= B) {
            cur = next_node;
            end = m;
        } else {
            cur = next_node + 1;
            start = m + 1;
            B -= seg_tree[next_node];
        }
    }

    seg_tree[cur] -= 1;

    return start;
}

void whenAisTwo(int B, int C) {
    int cur = 1;
    int start = 1, end = MAX;

    while (start < end) {
        seg_tree[cur] += C;
        int m = (start + end) / 2;

        int next_node = cur * 2;
        if (B <= m) {
            cur = next_node;
            end = m;
        } else {
            cur = next_node + 1;
            start = m + 1;
        } 
    }

    seg_tree[cur] += C;
}