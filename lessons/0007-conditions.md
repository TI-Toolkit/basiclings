# Intro to Condition Optimization
Conditions are ripe for optimization because TI-BASIC is very flexible with booleans.

Recall that TI-BASIC's conditionals and logical operators treat every nonzero real number as a "truthy" value and zero as a "falsy" value.

Recognizing this, use a logical operator to save two bytes off the provided program.
```json
{
  "id": 7,
  "requirements": [1],
  "name": "Intro to Condition Optimization",
  "starting_program": "If L1(2)=0\nDisp A",
  "required_savings": 2,
  "tests": [
    {
      "regex": "If not\\(L1\\(2[\\n:]Disp A"
    }
  ]
}
```