# Chapter 1: The Role of Algorithms in Computing

## Algorithms

**Instance of a problem:** consists of the input (satisfying whatever constraints are imposed in the problem statement) needed to compute a solution to the problem.

**Correct algorithm:** An algorithm for a computational problem is correct if, for every problem instance provided as input, it ***halts***—finishes its computing in finite time—and outputs the correct solution to the problem instance.

**Data structure:** is a way to store and organise data in order to facilitate access and modifications.

**Online algorithms:** which receive their input over time, rather than having all the input present at the start.

If computers were infinitely fast and memory was free, would you still need to study algorithms? Yes. Because we need to ensure the correctness of the algorithm still.

Computing time is a bounded resource.

Only decision problems (those with a yes/no answer) can be NP-complete.

# Chapter 2: Getting Started

## Insertion Sort

**Keys:** The numbers to be sorted.

**Satellite data**: Other data that are often associated with keys.

**Record:** A key and a satellite data together form a record.

**Loop invariant:** is a condition or property about a program's state that is guaranteed to be true before and after every single iteration of a loop

You need to show 3 things for loop invariant:
1. **Initialisation:** It is true prior to the first iteration of the loop.
2. **Maintenance:** If it is true before an iteration of the loop, it remains true before the next iteration.
3. **Termination:** The loop terminates, and when it terminates, the invariant—usually along with the reason that the loop terminated—gives us a useful property that helps show that the algorithm is correct.

Short circuiting

linear search

LINEAR-SEARCH(A, n, x)
for i = 1 to n
    if x == A[i]
        return i
return NIL

The invariant: At the start of each iteration of the `for` loop, the target value `x` is not present in the subarray `A[1 : i - 1]`.

Initialisation: Before the first iteration, `i = 1`. The subarray `A[1 : i - 1]` becomes `A[1 : 0]`, which is an empty subarray. The statement "`x` is not present in an empty subarray" is vacuously true. Therefore, the invariant holds before the loop begins.

Maintenance: At the start of iteration `i`, we know `x` is not in `A[1 : i - 1]` (by the invariant). During the loop, the code checks if `x == A[i]`. If the loop doesn't terminate, this condition must be false, meaning `x != A[i]`.

Now we know `x` is not in `A[1 : i - 1]` and `x` is not at `A[i]`. Combining these, `x` is not present in `A[1 : i]`. When the loop increments to $i' = i + 1$, our known clean zone is `A[1 : i' - 1]`. The invariant is perfectly maintained.

Termination: 

Branch 1: Early Termination (Element Found)
The loop terminates early at some index `i` because `x == A[i]`. The procedure returns `i`. This is correct because we found the exact index matching our target.

Branch 2: Natural Termination (Element Not Found)
The loop runs to completion. Just like our insertion sort problem, the loop terminates when `i` increments past the upper bound, meaning `$i = n + 1$`.Let's substitute `$i = n + 1$` into our invariant (`A[1 : i - 1]`): The invariant asserts that `x` is not present in the subarray `A[1 : n]`. Because `A[1 : n]` represents the entire array, we have mathematically proved that `x` does not exist anywhere in `A`. The code drops down to line 5 and returns `NIL`, which is completely correct.

ADD-BINARY-INTEGERS(A, B, n)
carry = 0
C = [0; n+1]
for i = 0 to n - 1
    current_sum = A[i] + B[i] + carry
    if  current_sum == 3
        C[i] = 1
        carry = 1
    else if current_sum == 2
        C[i] = 0
        carry = 1
    else if current_sum == 1
        C[i] = 1
        carry = 0
    else
        C[i] = 0
        carry = 0
C[n] = carry
return C

The variant: At the start of each loop iteration, `C[i + 2: n+1]` and 

## Analysing Algorithms

The RAM model assumptions:
- Each instruction takes the same amonunt of time as any other instruction
- Each data access—using the value of a variable or storing into a variable—takes the same amount of time as any other data access.
- Instructions: arithmetic (add, subtract, multiply, divide, remainder, floor, ceiling), data movement (load, store, copy), and control (conditional and unconditional branch, subroutine call and return)
- Data types: integer, floating point (for storing real-number approximations), and character.

Real computers do not usually have a separate data type for booleans TRUE and FALSE. Instead, they often test whether an integer value is 0 (FALSE) or nonzero (TRUE), as in C.


