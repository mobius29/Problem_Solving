#include <iostream>
#define sync()                                                                 \
  ios_base::sync_with_stdio(0);                                                \
  cin.tie(0)
using namespace std;

int X, N, a, b;

int main() {
  sync();
  cin >> X >> N;

  for (int i = 0; i < N; ++i) {
    cin >> a >> b;
    X -= (a * b);
  }

  cout << (X == 0 ? "Yes" : "No") << endl;

  return 0;
}
