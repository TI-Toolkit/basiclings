# Condition Optimizations: Use `sum(` (easy)
If all of the elements of a list are binary (0 or 1), you can use `sum(` to count how many are 1.

As a rare special case and illustrative example, `randBin(N,P)` is equivalent to `sum(P>rand(N`; i.e. it counts the number of times `P` is greater than the uniformly-distributed `rand` in a set of `N` samples.

In many golf problems, `randIntNoRep(` is useful when used in conjuction with `sum(`. Unlike the longer `cumSum(binomcdf(N-1,0` that produces a list of the numbers 1 through N in order, `randIntNoRep(1,N` produces the same numbers in a random order. We then apply some condition to every element of the list, and count the number of matches.

Use these techniques to count the number of distinct factors for a given integer 1<=N<=100.

_Hint: `fPart(`_
<!-- 23 bytes {
      "regex": "sum\\(not\\(fPart\\(N/randIntNoRep\\(1,N"
    }-->
```json
{
  "id": 11,
  "name": "Condition Optimizations: Use sum( (easy)",
  "requirements": [9],
  "starting_program": "0\nFor(I,1,100\nAns+(0=remainder(N,I\nEnd",
  "required_savings": 13,
  "tests": [
    [
      {
        "input": [{"name": "N", "value": 1}],
        "output": [{"name": "Ans", "value": 1}]
      },
      {
        "input": [{"name": "N", "value": 2}],
        "output": [{"name": "Ans", "value": 2}]
      },
      {
        "input": [{"name": "N", "value": 4}],
        "output": [{"name": "Ans", "value": 3}]
      },
      {
        "input": [{"name": "N", "value": 5}],
        "output": [{"name": "Ans", "value": 2}]
      },
      {
        "input": [{"name": "N", "value": 16}],
        "output": [{"name": "Ans", "value": 5}]
      },
      {
        "input": [{"name": "N", "value": 24}],
        "output": [{"name": "Ans", "value": 8}]
      },
      {
        "input": [{"name": "N", "value": 49}],
        "output": [{"name": "Ans", "value": 3}]
      },
      {
        "input": [{"name": "N", "value": 91}],
        "output": [{"name": "Ans", "value": 4}]
      },
      {
        "input": [{"name": "N", "value": 92}],
        "output": [{"name": "Ans", "value": 6}]
      },
      {
        "input": [{"name": "N", "value": 100}],
        "output": [{"name": "Ans", "value": 9}]
      }
    ]
  ]
}
```