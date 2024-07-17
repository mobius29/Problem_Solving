#include <iostream>
#include <map>
#define sync()                                                                 \
  ios_base::sync_with_stdio(0);                                                \
  cin.tie(0)
#define endl "\n"

using namespace std;

int N, a, b, answer;
map<int, bool> isEntered;

int main() {
  sync();
  cin >> N;

  for (int i = 0; i < N; ++i) {
    cin >> a >> b;

    bool isEnter = b == 1;

    if (isEntered[a] == isEnter) {
      answer += 1;
    }

    isEntered[a] = isEnter;
  }

  for (auto [id, status] : isEntered) {
    if (status)
      answer += 1;
  }

  cout << answer << endl;

  return 0;
}
