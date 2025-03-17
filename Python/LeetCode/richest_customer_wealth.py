# Richest customr wealth - 1672

# you are given m x n integer grid 'accounts' where accounts[i][j] is the amount of money the i-th customer has in the j-th bank. Return the welth that the richest customer has.
# A customer's welth is the amount of money they have in all their bank accounts. The richest customer is the cutomer that has the maximum wealth.

# Eg 1
# Inp: accounts = [[1,2,3], 3,2,1]
# output : 6

# Eg 2
# Inp: accounts = [[1,5],[7,3],[3,5]]
# out: 10


class Solution(object):
    def maximumWealth(self, accounts):
        """
        :type accounts: List[List[int]]
        :rtype: int
        """
        max_wealth = 0
        this_wealth = 0
        for m in accounts:
            this_wealth = 0
            for n in m:
                this_wealth += n
            if this_wealth > max_wealth:
                max_wealth = this_wealth

        # return type is INT
        return max_wealth


