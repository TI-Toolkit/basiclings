# Intro to List Optimization
Lists are a strength of TI-BASIC. The language supports many different ways of creating and manipulating lists and it is often possible to create, manipulate, and reduce a list all on one line.

We'll start this section by exploring different ways to create lists. Begin by creating a list of 9 zeros.

```json
{
  "id": 3,
  "name": "Intro to List Optimization",
  "brief_description": "Create a list with 9 zeros.",
  "requirements": [1],
  "starting_program": "{0,0,0,0,0,0,0,0,0}",
  "required_savings": 15,
  "tests": [
    {
      "regex": "0rand\\(9"
    },
    {
      "input": [],
      "output": [
        {
          "name": "Ans",
          "value": [0,0,0,0,0,0,0,0,0]
        }
      ]
    }
  ]
}
```