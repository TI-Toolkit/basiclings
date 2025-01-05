# One- and Two-Byte Tokens: String, Matrix, and List Names

It's easy to forget that built-in list, string, and matrix names are two bytes. There's an optimization to use these names as little as possible; can you spot it?

```json
{
  "id": 5,
  "name": "String, Matrix, and List Names (One- and Two-Byte Tokens)",
  "requirements": [2],
  "starting_program": "sum(L1->A\nLine(L1(1),L1(2),L1(3),L1(4",
  "required_savings": 2,
  "tests": [
    {
      "regex": "L1[\\n:]Line\\(Ans\\(1\\),Ans\\(2\\),Ans\\(3\\),Ans\\(4[\\n:]sum\\(Ans->A"
    }
  ]
}
```