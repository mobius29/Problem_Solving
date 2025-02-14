#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <utility>
#include <vector>
#define ll long long int
#define sync()                                                                 \
  ios_base::sync_with_stdio(0);                                                \
  cin.tie(0)

#define item pair<int, pair<ll, ll>>
using namespace std;

int N, K;
vector<item> list;

bool comp(item a, item b) {
  auto [a_cost, a_satisfy] = a.second;
  auto [b_cost, b_satisfy] = b.second;

  ll left = b_cost * a_satisfy;
  ll right = a_cost * b_satisfy;
  if (left == right) {
    if (a_cost == b_cost) {
      return a.first < b.first;
    }

    return a_cost < b_cost;
  }

  return left > right;
}

int main() {
  sync();
  cin >> N >> K;

  for (int i = 0; i < N; ++i) {
    int a;
    ll b, c;
    cin >> a >> b >> c;

    list.push_back({a, {b, c}});
  }

  sort(list.begin(), list.end(), comp);

  for (int i = 0; i < K; ++i) {
    cout << list[i].first << endl;
  }

  return 0;
}
