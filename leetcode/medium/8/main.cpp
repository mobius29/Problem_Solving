class Solution {
public:
    int myAtoi(string s) {
        long long int answer = 0;
        bool start_parsing = false;

        bool does_check_operator = false;
        bool is_negative = false;

        for (int i = 0; i < s.length(); ++i) {
            char c = s[i];
            
            if (!start_parsing && c == ' ') continue;
            start_parsing = true;

            if (!does_check_operator) {
                does_check_operator = true;

                if (c == '-') is_negative = true;
                if (c == '+' || c == '-') continue;
            }

            bool is_digit = '0' <= c && c <= '9';
            if (!is_digit) break;

            int digit = c - '0';
            if (is_negative) digit *= -1;

            answer = answer * 10 + digit;
            if (answer < INT_MIN) answer = INT_MIN;
            if (answer > INT_MAX) answer = INT_MAX;

            if (answer == INT_MIN || answer == INT_MAX) break;
        }

        return (int)answer;
    }
};