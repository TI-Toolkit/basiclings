# List Optimizations: `binompdf(`and`binomcdf(`
In addition to their expected uses in probability and statistics, the `binompdf(` and `binomcdf(` commands are useful in TI-BASIC when generating certain special lists.

Critically for optimization, `binompdf(N,0` produces a list with a single one, followed by N zeros. Also, in the two argument case, `binomcdf(` is, definitionally, `cumSum(binompdf(`.

We can put these two facts together in interesting ways:
- `cumSum(binomcdf(N,0` -> `{1,2,3,4,5,...,N+1`
- `cumSum(not(binompdf(N,0` -> `{0,1,2,3,4,...,N`

Expressing a sequence using `seq(` often takes more bytes than expressing the sequence using these techniques, especially if the trailing parentheses in the `binom` method can be pruned.

```json
{
  "id": 6,
  "name": "List Optimizations: binompdf( and binomcdf(",
  "requirements": [4],
  "starting_program": "15seq(X,X,1,10->L1\n{WHITE,LTGRAY,MEDGRAY,GRAY,DARKGRAY->L2",
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