#include <iostream>
#include <vector>
#define sync()                                                                 \
  ios_base::sync_with_stdio(0);                                                \
  cin.tie(0)
#define ends ' '
#define endl '\n'
using namespace std;

int N, M, r;
int recommended[101];

int main() {
  sync();
  cin >> N >> M;

  vector<int> current;
  while (M--) {
    cin >> r;

    recommended[r] += 1;
    if (recommended[r] == 1)
      current.push_back(r);

    int len = current.size();
    if (len > N) {
      int min_current_idx = 0;
      int min_recommended_idx = current[min_current_idx];
      int min_recommended_v = recommended[min_recommended_idx];

      for (int i = 1; i < N; ++i) {
        int cur_recommended_idx = current[i];
        int cur_recommended_v = recommended[cur_recommended_idx];

        if (cur_recommended_v < min_recommended_v) {
          min_current_idx = i;
          min_recommended_idx = cur_recommended_idx;
          min_recommended_v = cur_recommended_v;
        }
      }

      current.erase(current.begin() + min_current_idx);
      recommended[min_recommended_idx] = 0;
    }
  }

  for (int i = 1; i <= 100; ++i) {
    if (recommended[i] > 0)
      cout << i << ends;
  }

  cout << endl;

  return 0;
}
