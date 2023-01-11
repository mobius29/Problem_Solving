#include <bits/stdc++.h>
#define sync() ios_base::sync_with_stdio(0); cin.tie(0)
#define endl "\n"
#define ends " "
#define pb push_back
#define all(x) (x).begin(), (x).end()
#define getx(x, i) get<(x)>(i)
using namespace std;

typedef long long ll;
typedef pair<int ,int> pii;
typedef pair<ll, ll> pll;
typedef vector<int>  vi;
typedef vector<ll> vl;
typedef vector<pii> vii;
typedef vector<pll> vll;
typedef tuple<int, int, int> ti;
typedef tuple<ll, ll, ll> tl;

const int INF = 0x3f3f3f3f;
const ll LINF = 0x3f3f3f3f3f3f3f3f;

const int dx[] = {-1, 0, 1, 0};
const int dy[] = {0, -1, 0, 1};

void init(vl &hist, vi &tree, int node, int start, int end) {
  if (start == end) {
    tree[node] = start;
    return;
  }

  int m = (start + end) >> 1;
  init(hist, tree, node*2, start, m);
  init(hist, tree, node*2 + 1, m+1, end);

  ll leftHeight = hist[tree[node*2]];
  ll rightHeight = hist[tree[node*2+1]];

  tree[node] = leftHeight < rightHeight ? tree[node*2] : tree[node*2 + 1];
}

ll query(vl &hist, vi &tree, int node, int start, int end, int left, int right) {
  if (left > end || right < start)
    return -1;

  if (left <= start && end <= right)
    return tree[node];

  int m = (start + end) >> 1;
  long long leftChild = query(hist, tree, node*2, start, m, left, right);
  long long rightChild = query(hist, tree, node*2 + 1, m + 1, end, left, right);
  
  if (leftChild == -1) return rightChild;
  if (rightChild == -1) return leftChild;
  
  return hist[leftChild] < hist[rightChild] ? leftChild : rightChild;
}

ll getArea(vl &hist, vi &tree, int start, int end) {
  int n = hist.size();
  int idx = query(hist, tree, 1, 0, n - 1, start, end);

  ll area = hist[idx] * (end - start + 1);

  if(idx > start) {
    ll leftArea = getArea(hist, tree, start, idx-1);
    area = max(area, leftArea);
  }

  if(idx < end) {
    ll rightArea = getArea(hist, tree, idx+1, end);
    area = max(area, rightArea);
  }

  return area;
}

int main () {
    sync();
    while(true) {
      int n; cin >> n;
      if (n == 0) break;

      int h = (int)ceil(log2(n));
      vl hist(n, 0);
      vi tree(1 << (h + 1), 0);

      for(int i = 0; i < n; ++i)
        cin >> hist[i];
      init(hist, tree, 1, 0, n - 1);

      cout << getArea(hist, tree, 0, n-1) << endl;
    }

    return 0;
}