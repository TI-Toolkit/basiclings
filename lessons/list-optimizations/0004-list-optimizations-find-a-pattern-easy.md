# Find a Pattern (easy)
List constants often have patterns. Ask "what do the elements of this list have in common?" or "can I express this as a sequence?".

_Hint: Remember that the color constants are just numbers. What operations can you do on lists of numbers?_

```json
{
  "id": 4,
  "name": "List Optimizations: Find a Pattern (easy)",
  "requirements": [3],
  "starting_program": "{RED,ORANGE,YELLOW,GREEN,BLUE,MAGENTA->L1\n{0,3,24,81,192,375,648,1029->L2",
  "required_savings": 20,
  "tests": [
    {
      "regex": "10\\+{1,5,9,4,0,3->L1[\\n:]seq\\(3([A-Z]|theta)\\^\\^3,\\1,0,7->L2"
    },
    {
      "regex": "seq\\(3([A-Z]|theta)\\^\\^3,\\1,0,7->L2[\\n:]10\\+{1,5,9,4,0,3->L1"
    },
    {
      "input": [{"name": "L3", "value": [0,3,24,81,192,375,648,1029]}],
      "output": [
        {
          "name": "L1",
          "value": [11,15,19,14,10,13]
        },
        {
          "name": "L2",
          "value": [0,3,24,81,192,375,648,1029]
        }
      ]
    }
  ]
}
```