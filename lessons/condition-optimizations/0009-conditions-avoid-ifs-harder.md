# Condition Optimizations: Avoid Ifs (harder)
Try to find a one-liner for this alternative character movement scheme.
```
X+(V>0)-(V<0->X
If X>10
1->X
If X<1
10->X
```

_Hint: Find a transcendental function in the catalog that maps large positive values to 1, and large negative values to -1._
```json
{
  "id": 9,
  "name": "Condition Optimizations: Avoid Ifs (harder)",
  "starting_program": "X+(V>0)-(V<0->X\nIf X>10\n1->X\nIf X<1\n10->X",
  "requirements": [8],
  "required_savings": 15,
  "tests": [
    {
      "input": [{"name": "X", "value": 1}, {"name": "V", "value": -2}],
      "output": [{"name": "X", "value": 10}]
    },
    {
      "input": [{"name": "X", "value": 1}, {"name": "V", "value": 3}],
      "output": [{"name": "X", "value": 2}]
    },
    {
      "input": [{"name": "X", "value": 10}, {"name": "V", "value": 7}],
      "output": [{"name": "X", "value": 1}]
    },
    {
      "input": [{"name": "X", "value": 10}, {"name": "V", "value": -1}],
      "output": [{"name": "X", "value": 9}]
    },
    {
      "input": [{"name": "X", "value": 4}, {"name": "V", "value": 0}],
      "output": [{"name": "X", "value": 4}]
    }
  ]
}
```