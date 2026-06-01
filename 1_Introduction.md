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

### Analysis of insertion sort

The best notion for ***input size*** depends on the problem being studied. For many problems, such as sorting or computing discrete Fourier transforms, the most natural measure is the *number of items in the input*.

For many other problems, such as multiplying two integers, the best measure of input size is the *total number of bits* needed to represent the input in ordinary binary notation.

Sometimes it is more appropriate to describe the size of the input with more than just one number. For example, if the input to an algorithm is a graph, we usually characterise the input size by both the number of vertices and the number of edges in the graph.

The ***running time*** of an algorithm on a particular input is the number of instructions and data accesses executed.

We will adopt the view: A constant amount of time is required to execute each line of our pseudocode. One line might take more or less time than another line, but we'll assume that each execution of the $k$th line takes $c_k$ time, where $c_k$ is a constant.

The running time of an algorithm is the sum of running times for each statement executed. A statement that takes $c_k$ steps to execute and executes $m$ times contributes $c_k * m$ to the total running time.

We usually denote the running time of an algorithm on an input of size $n$ by $T(n)$.

$$
\begin{array}{llll}
1 & \textbf{for } i = 2 \textbf{ to } n & c_1 & n \\
2 & \quad \textit{key} = A[i] & c_2 & n - 1 \\
3 & \quad \color{brown}{\text{// Insert } A[i] \text{ into the sorted subarray } A[1 : i - 1].} & 0 & n - 1 \\
4 & \quad j = i - 1 & c_4 & n - 1 \\
5 & \quad \textbf{while } j > 0 \textbf{ and } A[j] > \textit{key} & c_5 & \sum_{i=2}^{n} t_i \\
6 & \qquad A[j + 1] = A[j] & c_6 & \sum_{i=2}^{n} (t_i - 1) \\
7 & \qquad j = j - 1 & c_7 & \sum_{i=2}^{n} (t_i - 1) \\
8 & \quad A[j + 1] = \textit{key} & c_8 & n - 1
\end{array}
$$

#### Total running time

$$T(n) = c_{1}n + c_{2}(n-1) + c_{4}(n-1) + c_{5}\sum_{i=2}^{n}t_i + c_{6}\sum_{i=2}^{n}(t_i - 1) + c_{7}\sum_{i=2}^{n}(t_i - 1) + c_8(n - 1)$$

#### Best case scenario
The best case scenario in our insertion-sort algorithm is when the input is already in the correct order. In that case:

$$
\begin{aligned}
    T(n) &= c_{1}n + c_{2}(n-1) + c_{4}(n-1) + c_{5}(n-1) + c_{8}(n-1) \\
    &= (c_1 + c_2 + c_4 + c_5 + c_8)n - (c_2 + c_4 + c_5 + c_8)
\end{aligned}
$$

We can express this running time as $an + b$ for *constants* $a$ and $b$ that depend on the statement costs $c_k$ (where $a = c_1 + c_2 + c_4 + c_5 + c_8$ and $b = c_2 + c_4 + c_5 + c_8$). The running time is thus a ***linear function*** of $n$.

#### Worst case scenario

The worst case scenario arises when the array is in reverse sorted order. The procedure must compare each element `A[i]` with each element in the entire sorted subarray `A[1:i-1]`, and so $t_i=i$ for $i = 2, 3, \ldots , n$.

Nothing that

$$
\begin{aligned}
    \sum_{i=2}^{n}i &= \left(\sum_{i=1}^{n}i\right) - 1 \\
                    &= \frac{n(n+1)}{2} - 1
\end{aligned}
$$

and

$$
\begin{aligned}
    \sum_{i=2}^{n}(i-1) &= \sum_{i=1}^{n-1}i \\
                    &= \frac{n(n-1)}{2}
\end{aligned}
$$

we find that in the worst case, the running time of INSERTION-SORT is


$$
\begin{aligned}
T(n) = & \; c_1n + c_2(n-1) + c_4(n-1) + c_5\left(\frac{n(n+1)}{2} - 1\right) \\
& + c_6\left(\frac{n(n-1)}{2}\right) + c_7\left(\frac{n(n-1)}{2}\right) + c_8(n-1) \\
= & \; \left(\frac{c_5}{2} + \frac{c_6}{2} + \frac{c_7}{2}\right)n^2 + \left(c_1 + c_2 + c_4 + \frac{c_5}{2} - \frac{c_6}{2} - \frac{c_7}{2} + c_8\right)n \\
& - (c_2 + c_4 + c_5 + c_8).
\end{aligned}
$$

We can express this worst-case running time as $an^2 + bn + c$ for constants $a$, $b$, and $c$ that again depends on the statement costs $c_k$ (now, $a = c_5/2 + c_6/2 + c_7/2$, $b = c_1 + c_2 + c_4 + c_5/2 - c_6/2 - c_7/2 + c8$, and $c = -(c_2 + c_4 + c_5 + c_8)$). The running time is thus a ***quadratic function*** of $n$.

Typically, as in insertion sort, the running time of an algorithm is fixed for a given input (if not a randomised algorithm).

### Worst-case and average-case analysis

Why we will mostly care about the ***worst-case running time***:
- Gives an upper bound on the running time of *any* input which gives a guarantee that the algorithm never takes any longer. Especially important for real-time computing, in which operations must complete by a deadline.
- For some algorithms, the worst case happens fairly often e.g., searching a database for sth and it doesn't exist in the db.
- The *average case* is often roughly as bad ass the worst case.

### Order of growth

We'll make another simplifying abstraction: it is the ***rate of growth***, or ***order of growth***, of the running time that really interests us. We therefore only consider the leading term of a formula, since the lower-order terms are relatively insignificant for large values of $n$. We also ignore the leading term's constant coefficient, since constant factors are less significant than the rate of groth in determining computational efficiency for large inputs.


