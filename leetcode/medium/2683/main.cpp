class Solution {
public:
  bool doesValidArrayExist(vector<int> &derived) {
    int size = derived.size();
    int v = 1;

    for (int i = 0; i < size; ++i) {
      v ^= derived[i];
    }

    return v;
  }
};
