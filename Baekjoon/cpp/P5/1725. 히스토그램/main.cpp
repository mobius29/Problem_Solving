#include <bits/stdc++.h>
#define sync() ios_base::sync_with_stdio(0); cin.tie(0)
#define endl "\n"
#define ends " "
#define fs first
#define se second
#define pb push_back
#define all(x) (x).begin, (x).end()
#define getx(x, i) get<(x)>(i)
using namespace std;

typedef long long ll;
typedef pair<int ,int> pii;
typedef pair<ll, ll> pll;
typedef vector<int>  vi;
typedef vector<ll> vl;
typedef vector<bool> vb;
typedef vector<pii> vii;
typedef vector<pll> vll;
typedef tuple<int, int, int> ti;
typedef tuple<ll, ll, ll> tl;

const int INF = 0x3f3f3f3f;
const ll LINF = 0x3f3f3f3f3f3f3f3f;

const int dx[] = {-1, 0, 1, 0};
const int dy[] = {0, -1, 0, 1};

int N;
vi hist, tree;

void init(int node, int start, int end) {
    if (start == end) {
        tree[node] = start;
        return ;
    }

    int m = (start + end) >> 1;
    init(node * 2, start, m);
    init(node * 2 + 1, m + 1, end);

    int left = hist[tree[node * 2]];
    int right = hist[tree[node * 2 + 1]];
    tree[node] = (left < right) ? tree[node * 2] : tree[node * 2 + 1];
}

int query (int node, int start, int end, int left, int right) {
    if (left > end || right < start) return -1;
    if (left <= start && end <= right) return tree[node];

    int m = (start + end) >> 1;
    int leftChild = query(node * 2, start, m, left, right);
    int rightChild = query(node * 2 + 1, m + 1, end, left, right);

    if (leftChild == -1) return rightChild;
    if (rightChild == -1) return leftChild;

    return hist[leftChild] < hist[rightChild] ? leftChild : rightChild;
}

int getArea(int left, int right) {
    int idx = query(1, 0, N - 1, left, right);
    int area = (right - left + 1) * hist[idx];

    if (left < idx) {
        int temp = getArea(left, idx - 1);
        area = max(area, temp);
    }

    if (idx < right) {
        int temp = getArea(idx + 1, right);
        area = max(area, temp);
    }

    return area;
}

int main() {
    sync(); cin >> N;
    hist.assign(N, 0);

    int height = (int)ceil(log2(N));
    int tree_size = 1 << (height + 1);
    tree.assign(tree_size, 0);

    for (int i = 0; i < N; ++i) cin >> hist[i];

    init(1, 0, N - 1);

    cout << getArea(0, N - 1) << endl;    
}

/*
예제 입력: 7 2 1 4 5 1 3 3
예제 출력: 8
*/