#include <iostream>
#include <vector>
#define sync()                                                                 \
  ios_base::sync_with_stdio(0);                                                \
  cin.tie(0)
#define pii pair<int, int>
#define endl "\n"
#define ends " "
using namespace std;

int N;
char c;
vector<pii> gomgom;

int main() {
  sync();

  cin >> N;

  int min_horizontal = N, max_horizontal = -1;
  int min_vertical = N, max_vertical = -1;

  for (int i = 0; i < N; ++i) {
    for (int j = 0; j < N; ++j) {
      cin >> c;

      if (c == 'G') {
        min_horizontal = min(min_horizontal, j);
        max_horizontal = max(max_horizontal, j);

        min_vertical = min(min_vertical, i);
        max_vertical = max(max_vertical, i);
      }
    }
  }

  int answer = 0;
  if (min_horizontal != max_horizontal)
    answer += min(max_horizontal, N - 1 - min_horizontal);

  if (min_vertical != max_vertical)
    answer += min(max_vertical, N - 1 - min_vertical);

  cout << answer << endl;

  return 0;
}
