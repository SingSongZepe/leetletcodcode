292. Nim Game
     Easy
     Topics
     Companies
     Hint
     You are playing the following Nim Game with your friend:

Initially, there is a heap of stones on the table.
You and your friend will alternate taking turns, and you go first.
On each turn, the person whose turn it is will remove 1 to 3 stones from the heap.
The one who removes the last stone is the winner.
Given n, the number of stones in the heap, return true if you can win the game assuming both you and your friend play optimally, otherwise return false.



Example 1:

Input: n = 4
Output: false
Explanation: These are the possible outcomes:
1. You remove 1 stone. Your friend removes 3 stones, including the last stone. Your friend wins.
2. You remove 2 stones. Your friend removes 2 stones, including the last stone. Your friend wins.
3. You remove 3 stones. Your friend removes the last stone. Your friend wins.
   In all outcomes, your friend wins.
   Example 2:

Input: n = 1
Output: true
Example 3:

Input: n = 2
Output: true


Constraints:

1 <= n <= 231 - 1



# Game Theory
Hello my friends. I come up with a rigorous proof that "return (n % 4)" works. Here is my detailed explanation and I apologize if this is wordy or more complex than your own method.

Statement1
we have a function Win(n) that takes in an input "n", and outputs: 0 or 1.
For its output 1, it means "you can win in one of the three moves: take 1, 2, 3"
For its output 0, it means "you cannot win, no matter what moves you take" (in the assumption that your opponent is smart).

Statement2
Now I claim that the function Win(n) is:

1 when n % 4 != 0

0 when n % 4 == 0

In other words, n = 4k + 4, then Win(n) = 0; n = 4k+1, 4k+2, 4k+3, Win(n) = 1;

Now I prove this by Induction:

when k = 0.
n = 1, 2, 3, on every of these three case you can win by taking all of them.

n = 4, you simply cannot win in any way.

Assume this is true for k = x:
n = 4x+1, 4x+2, 4x+3, Win(n) = 1; n = 4x + 4, then Win(n) = 0;
Then I will prove the following:
for k = x + 1, this is also true:
n = 4x+5, 4x+6, 4x+6, Win(n) = 1; n = 4x + 8, then Win(n) = 0;
For n = 4x+5, you can take 1 to leave your opponent n=4x+4, which is 0 in the assumption. Similarly, for n=4x+6, 4x+7, you should take 2, 3 stones from it.
For n = 4x+8, any number of stone (1, 2, 3) you take, will leave your opponent the previous three cases, which in turn leaves you a 4x+4 case, and you lose.
In conclusion, the Win(n) function is:

Win(n) = 0; n = 4k + 4 (k is int)

Win(n) = 1; n = 4k+1, 4k+2, 4k+3 (k is int)

And that is why return n % 4 works.
