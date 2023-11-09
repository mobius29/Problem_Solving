/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode *ret = new ListNode();

        ListNode* cur = ret;
        int carry = 0;
        while (l1 != nullptr || l2 != nullptr) {
            int l1_value = l1 != nullptr ? l1->val : 0;
            int l2_value = l2 != nullptr ? l2->val : 0;

            int sum = l1_value + l2_value + carry;
            
            int value = sum % 10;
            carry = sum / 10;

            ListNode *newNode = new ListNode(value);

            cur->next = newNode;
            cur = cur->next;

            if (l1 != nullptr) l1 = l1->next;
            if (l2 != nullptr) l2 = l2->next;
        }

        if (carry == 1) {
            ListNode *newNode = new ListNode(1);
            cur->next = newNode;
        }

        ListNode *temp = ret;
        ret = ret->next;
        delete(temp);

        return ret;
    }
};