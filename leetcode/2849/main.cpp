#include <stdlib.h>
#include <algorithm>

class Solution {
public:
    bool isReachableAtTime(int sx, int sy, int fx, int fy, int t) {
        if (sx == fx && sy == fy) return t != 1;

        bool _x = -t <= (fx - sx) && (fx - sx) <= t;
        bool _y = -t <= (fy - sy) && (fy - sy) <= t;
        
        return _x && _y;
    }
};