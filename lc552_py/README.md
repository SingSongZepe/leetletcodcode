552. Student Attendance Record II
Hard
Topics
Companies
An attendance record for a student can be represented as a string where each character signifies whether the student was absent, late, or present on that day. The record only contains the following three characters:

'A': Absent.
'L': Late.
'P': Present.
Any student is eligible for an attendance award if they meet both of the following criteria:

The student was absent ('A') for strictly fewer than 2 days total.
The student was never late ('L') for 3 or more consecutive days.
Given an integer n, return the number of possible attendance records of length n that make a student eligible for an attendance award. The answer may be very large, so return it modulo 109 + 7.

 

Example 1:

Input: n = 2
Output: 8
Explanation: There are 8 records with length 2 that are eligible for an award:
"PP", "AP", "PA", "LP", "PL", "AL", "LA", "LL"
Only "AA" is not eligible because there are 2 absences (there need to be fewer than 2).
Example 2:

Input: n = 1
Output: 3
Example 3:

Input: n = 10101
Output: 183236316
 

Constraints:

1 <= n <= 105

# Transition Solution
### Intuition
We can model the process as a state transition system. Initially we have 3 states: Absence (AAA), Presence (PPP), and Late (LLL). Given the restrictions on the choices between them, we need to add some extra states to the model.

We only need to define a transition matrix and an initial state. By raising the matrix to the power of NNN, we can describe the pairwise number of transitions from each state to each other.

Having built the matrix, we should declare a vector of possible initial states. The final result can be calculated as the sum of the cells of the resulting matrix A=initial_state×transitionsNA = initial\_state \times transitions^NA=initial_state×transitions 
N
 .

Approach
We start with the three primary states: AAA, PPP, and LLL. To fulfill the requirements, we need to add some more states:

To manage the cases when LLL comes after another LLL, we add the state LLLLLL: this way we allow a maximum of 2 LLL in a row, and from this state, you can only transition to states AAA or PPP.
Since AAA can only occur once, we can double the states PPP, LLL, and LLLLLL to have states PAP^{A}P 
A
 , LAL^{A}L 
A
 , LLALL^{A}LL 
A
 . These states indicate that we've already encountered AAA before. State AAA will always transition to one of them, and these new states will not transition back to AAA.
Summing it up, we have 7 states: AAA, PPP, LLL, LLLLLL, PAP^{A}P 
A
 , LAL^{A}L 
A
 , LLALL^{A}LL 
A
 .

Transition Matrix
AAA leads to "marked with A" states PAP^{A}P 
A
 , LAL^{A}L 
A
 .
PPP allows transitions to states AAA, LLL, and itself PPP.
From LLL, we can transition to states AAA, PPP, and LLLLLL.
From LLLLLL, only to AAA and PPP.
PAP^{A}P 
A
  transitions to states LAL^{A}L 
A
  and PAP^{A}P 
A
  itself.
LAL^{A}L 
A
  transitions to LLALL^{A}LL 
A
  or PAP^{A}P 
A
 .
From LLALL^{A}LL 
A
 , we must go to PAP^{A}P 
A
 .
Thus, we come up with the following matrix (rows mean "from", columns mean "to"):

∅\empty∅	AAA	PPP	LLL	LLLLLL	PAP^{A}P 
A
 	LAL^{A}L 
A
 	LLALL^{A}LL 
A
 
AAA	0	0	0	0	1	1	0
PPP	1	1	1	0	0	0	0
LLL	1	1	0	1	0	0	0
LLLLLL	1	1	0	0	0	0	0
PaP^{a}P 
a
 	0	0	0	0	1	1	0
LaL^{a}L 
a
 	0	0	0	0	1	0	1
LLaLL^{a}LL 
a
 	0	0	0	0	1	0	0
In our case, we start either from AAA, PPP, or LLL. Thus the initial_state=[1,1,1,0,0,0,0]initial\_state = [1, 1, 1, 0, 0, 0, 0]initial_state=[1,1,1,0,0,0,0].

To efficiently compute potentially high powers of such a matrix, we use Exponentiation by squaring.

Complexity
Time complexity: O(log⁡(n))O(\log(n))O(log(n))
The time complexity is O(log⁡(n)⋅k3)O(\log(n) \cdot k^3)O(log(n)⋅k 
3
 ), where kkk is the number of states (7 in this case). Thus time complexity is O(343)O(343)O(343). This is because matrix multiplication takes O(k3)O(k^3)O(k 
3
 ) time, and we perform it O(log⁡(n))O(\log(n))O(log(n)) times.
Space complexity: O(1)O(1)O(1)
The space complexity is O(k2)O(k^2)O(k 
2
 ) or O(49)O(49)O(49) due to the space required to store the matrices.