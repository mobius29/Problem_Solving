#include <iostream>
#include <queue>
#include <vector>
#define sync()                                                                 \
  ios_base::sync_with_stdio(0);                                                \
  cin.tie(0)
#define endl "\n"
#define ends " "
#define MAX 1001
using namespace std;

int N, M, cnt, num, cur, indegree[MAX];
vector<int> answer, graph[MAX];

bool topological_sort() {
  queue<int> q;

  for (int i = 1; i <= N; ++i) {
    if (indegree[i] == 0)
      q.push(i);
  }

  for (int i = 0; i < N; ++i) {
    if (q.empty())
      return false;

    int fr = q.front();
    q.pop();

    answer.push_back(fr);
    for (auto next : graph[fr]) {
      indegree[next] -= 1;
      if (indegree[next] == 0) {
        q.push(next);
      }
    }
  }

  return true;
}

int main() {
  sync();
  cin >> N >> M;

  for (int i = 0; i < M; ++i) {
    cin >> cnt >> cur;

    for (int j = 1; j < cnt; ++j) {
      cin >> num;

      graph[cur].push_back(num);
      indegree[num] += 1;

      cur = num;
    }
  }

  int result = topological_sort();
  if (!result) {
    cout << 0 << endl;
  } else {
    for (auto node : answer)
      cout << node << endl;
  }

  return 0;
}
