# Intro to One- and Two-Byte Tokens
Even though this course has you type TI-BASIC code character-by-character on your computer, it is essential to remember that TI-BASIC is instead constructed out of *tokens*, with each token requiring either one or two bytes. Expert TI-BASIC programmers can scan a line of code and sum up exactly how many bytes that line costs. In the words of legendary golfer lirtosiast, you should "know the sizes of the most common tokens so you can golf in your head while cooking, taking a shower, etc."

TI managed to fit __243__ tokens in one byte. In my experience, it is far easier to memorize these one-byte tokens than to attempt to remember all of the two-byte tokens; there are strong patterns in the one-byte tokens that aid memorization.

Most of the tokens relating to numbers are one byte. However, there is a notable and common exception; identify and remove it from the expression.

_Hint: The expression will not have valid language syntax after you remove this token._

```json
{
  "id": 2,
  "name": "Intro to One- and Two-Byte Tokens",
  "requirements": [0],
  "starting_program": "[|e]^pi[i]=~1",
  "required_savings": 2,
  "tests": [
    {
      "regex": "\\^pi\\[i\\]=~1"
    }
  ]
}
```