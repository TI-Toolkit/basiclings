# List Optimizations: `binompdf(` and `binomcdf(`
In addition to their expected uses in probability and statistics, the `binompdf(` and `binomcdf(` commands are useful in TI-BASIC because of a degenerate case: `binompdf(N,0` produces a list with a single one, followed by N zeros. In the two argument case, `binomcdf(` is, definitionally, `cumSum(binompdf(`.

We can put these two facts together in interesting ways:
- `cumSum(binomcdf(N,0` -> `{1,2,3,4,5,...,N+1`
- `cumSum(not(binompdf(N,0` -> `{0,1,2,3,4,...,N`

This provides a concise way to obtain a list with *sequential* integers.

Expressing a sequence using `seq(` often takes more bytes than expressing the same sequence using these techniques, especially if the trailing parentheses in the `binom` methods can be pruned. Try your hand at a couple of straightforward examples.

```json
{
  "id": 6,
  "name": "List Optimizations: binompdf( and binomcdf(",
  "requirements": [4],
  "starting_program": "seq(15X,X,1,10->L1\n{WHITE,LTGRAY,MEDGRAY,GRAY,DARKGRAY->L2",
  "required_savings": 7,
  "tests": [
    [
      {
        "input": [],
        "output": [
          {
            "name": "L1",
            "value": [15,30,45,60,75,90,105,120,135,150]
          }
        ]
      },
      {
        "input": [],
        "output": [
          {
            "name": "L2",
            "value": [20,21,22,23,24]
          }
        ]
      }
    ]
  ]
}
```