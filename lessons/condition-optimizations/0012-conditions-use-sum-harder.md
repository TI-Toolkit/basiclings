# Condition Optimizations: Use `sum(` (harder)
Recall that every number has a unique octal (base-8) representation. Count the number of nonzero digits in this representation for a number 0 <= X <= 99999999 (base-10).
<!-- this puzzle is inspired by a problem kerm posed & the main golf ideas here are due to lirto: https://www.cemetech.net/forum/viewtopic.php?p=236371#236371 -->
```json
{
  "id": 12,
  "name": "Condition Optimizations: Use sum( (harder)",
  "short_description": "Count the number of nonzero digits in the octal representation of a number 0 <= X <= 99999999 (base-10).",
  "starting_program": "0->C\nFor(I,0,1+logBASE(1+X,8\nremainder(X,8^I->R\nIf R\nC+1->C\nX-R->X\nEnd\nC",
  "required_savings": 33,
  "requirements": [11],
  "tests": [
    [
      {
        "input": [{"name": "X", "value": 0}],
        "output": [{"name": "Ans", "value": 0}]
      },
      {
        "input": [{"name": "X", "value": 1}],
        "output": [{"name": "Ans", "value": 1}]
      },
      {
        "input": [{"name": "X", "value": 8}],
        "output": [{"name": "Ans", "value": 1}]
      },
      {
        "input": [{"name": "X", "value": 64}],
        "output": [{"name": "Ans", "value": 1}]
      },
      {
        "input": [{"name": "X", "value": 640}],
        "output": [{"name": "Ans", "value": 2}]
      },
      {
        "input": [{"name": "X", "value": 134}],
        "output": [{"name": "Ans", "value": 2}]
      },
      {
        "input": [{"name": "X", "value": 136}],
        "output": [{"name": "Ans", "value": 2}]
      },
      {
        "input": [{"name": "X", "value": 80888}],
        "output": [{"name": "Ans", "value": 5}]
      },
      {
        "input": [{"name": "X", "value": 21777344}],
        "output": [{"name": "Ans", "value": 6}]
      },
      {
        "input": [{"name": "X", "value": 47640080}],
        "output": [{"name": "Ans", "value": 7}]
      },
      {
        "input": [{"name": "X", "value": 31777343}],
        "output": [{"name": "Ans", "value": 8}]
      },
      {
        "input": [{"name": "X", "value": 99999999}],
        "output": [{"name": "Ans", "value": 8}]
      },
      {
        "input": [{"name": "X", "value": 21380977}],
        "output": [{"name": "Ans", "value": 9}]
      }
    ]
  ]
}
```