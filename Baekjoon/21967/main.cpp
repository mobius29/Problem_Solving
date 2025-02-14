#include <algorithm>
#include <cstdlib>
#include <iostream>
#define sync()                                                                 \
  ios_base::sync_with_stdio(0);                                                \
  cin.tie(0)
#define endl "\n"
using namespace std;

int N, cur, answer;
vector<int> arr;

int main() {
  sync();
  cin >> N;

  for (int i = 0; i < N; ++i) {
    cin >> cur;
    arr.push_back(cur);
  }

  int left = 0, right = 0;
  int min_v = arr[0], max_v = arr[0];

  answer = 1;
  while (left <= right) {
    right += 1;
    if (right >= N)
      break;

    max_v = max(max_v, arr[right]);
    min_v = min(min_v, arr[right]);

    if (max_v - min_v <= 2) {
      answer = max(answer, right - left + 1);
      continue;
    }

    for (int x = right - 1; x >= left; --x) {
      if (abs(arr[right] - arr[x]) > 2) {
        left = x + 1;
        break;
      }
    }

    int new_max_v = arr[left], new_min_v = arr[left];
    for (int x = left; x <= right; ++x) {
      new_max_v = max(new_max_v, arr[x]);
      new_min_v = min(new_min_v, arr[x]);
    }

    max_v = new_max_v;
    min_v = new_min_v;
  }

  cout << answer << endl;

  return 0;
}
