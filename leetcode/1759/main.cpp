const int MOD = 1e9 + 7;

class Solution {
public:
    int sum(int count) {
        long long int ret = (1 + count);
        ret *= (count / 2);
        if (count % 2 == 1) ret += (count / 2) + 1;

        return (int)(ret % MOD);
    }

    int countHomogenous(string s) {
        int count = 1, answer = 0;
        for (int i = 1; i < s.length(); ++i) {
            if (s[i] == s[i-1]) {
                ++count;
                continue;
            }

            answer += sum(count);
            answer %= MOD;

            count = 1;
        }
        answer += sum(count);
        answer %= MOD;

        return answer;
    }
};