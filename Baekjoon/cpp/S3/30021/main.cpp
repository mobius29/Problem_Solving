#include <iostream>
#include <vector>
#define sync()                                                                 \
  ios_base::sync_with_stdio(0);                                                \
  cin.tie(0)
#define MAX 5001
#define ends " "
#define endl "\n"
using namespace std;

int N;
vector<int> answer;
bool checked[MAX];

bool is_prime(int n) {
  if (n == 1)
    return false;

  for (int i = 2; i * i <= n; ++i) {
    if (n % i == 0)
      return false;
  }

  return true;
}

bool solve(int cur_sum) {
  if (answer.size() == N) {
    return true;
  }

  for (int i = 1; i <= N; ++i) {
    if (checked[i])
      continue;

    int next_sum = cur_sum + i;
    if (is_prime(next_sum))
      continue;

    checked[i] = true;
    answer.push_back(i);
    bool ret = solve(next_sum);
    if (ret)
      return ret;
    answer.pop_back();
    checked[i] = false;
  }

  return false;
}

int main() {
  sync();
  cin >> N;

  bool flag = false;
  for (int i = 1; i <= N; ++i) {
    if (is_prime(i))
      continue;

    checked[i] = true;
    answer.push_back(i);
    flag = solve(i);
    if (flag)
      break;
    answer.pop_back();
    checked[i] = false;
  }

  if (flag) {
    cout << "YES" << endl;
    for (auto i : answer) {
      cout << i << ends;
    }
    cout << endl;
  } else {
    cout << "NO" << endl;
  }

  return 0;
}
