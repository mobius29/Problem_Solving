#include <algorithm>

class Solution {
public:
    int eliminateMaximum(vector<int>& dist, vector<int>& speed) {
        vector<int> times;
        for (int i = 0; i < dist.size(); ++i) {
            int time = dist[i] / speed[i];
            if (dist[i] % speed[i] != 0) time += 1;

            times.push_back(time);
        }

        sort(times.begin(), times.end());

        int eliminated_monsters = 1;
        for (int i = 1; i < times.size(); ++i) {
            if (times[i] - i <= 0) break;
            eliminated_monsters += 1;
        }

        return eliminated_monsters;
    }
};