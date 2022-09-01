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

int N;

int main () {
    sync(); cin >> N;
    priority_queue<int> min_heap, max_heap;

    for(int i = 0; i < N; ++i) {
      int num = 0; cin >> num;

      if(max_heap.size() == min_heap.size()) max_heap.push(num);
      
      else min_heap.push(-num);

      if(min_heap.size() && max_heap.top() > -min_heap.top()) {
        int max_top = max_heap.top(); max_heap.pop();
        int min_top = -min_heap.top(); min_heap.pop();

        max_heap.push(min_top);
        min_heap.push(-max_top);
      }

      cout << max_heap.top() << endl;
    }

    return 0;
}