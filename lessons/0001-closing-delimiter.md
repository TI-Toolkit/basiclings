# Closing Delimiter Optimizations
Closing parentheses, brackets, braces, and quotes are optional in three situations:
 - At the end of a line
 - Before a store arrow
 - At the end of a string that will be evaluated as an expression (eg. `expr(`, attached list formula, equation variable)

Closing delimiter optimizations are among the most plentiful optimizations in regular programs. Occasionally, you will have to manipulate an expression to maximize the savings.

```json
{
  "id": 1,
  "name": "Closing Delimiters",
  "requirements": [0],
  "starting_program": "sum(L1)/5->A\nIf L1(2)<A or B\nDisp A",
  "required_savings": 2,
  "tests": [
    {
      "regex": "\\.2sum\\(L1\\->A[\\n:]If B or A>L1\\(2[\\n:]Disp A"
    }
  ]
}
```