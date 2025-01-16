# Condition Optimizations: Avoid Ifs (easy)
Like some other languages, TI-BASIC uses 1 and 0 to represent boolean `true` and `false`. Because of this, we can often use conditions inside of expressions to replace if statements:
```
If not(A
B+1->B
```
can become
```
B+not(A->B
```
<!-- Too many possibilities to test with regex... just better to behavior-test -->
```json
{
  "id": 8,
  "name": "Condition Optimizations: Avoid Ifs",
  "requirements": [7],
  "brief_description": "Optimize this character movement routine", 
  "starting_program": "If K=24\nI-1->I\nIf K=26\nI+1->I\nIf K=25\nJ-1->J\nIf K=34\nJ+1->J\nIf I<0\n0->I\nIf I>10\n10->I\nIf J<0\n0->J\nIf J>10\n10->J",
  "required_savings": 40,
  "tests": [
    [
      {
        "input": [
          {"name": "K", "value": 24},
          {"name": "I", "value": 0},
          {"name": "J", "value": 5}
        ],
        "output": [
          {"name": "I", "value": 0},
          {"name": "J", "value": 5}
        ]
      },
      {
        "input": [
          {"name": "K", "value": 24},
          {"name": "I", "value": 1},
          {"name": "J", "value": 5}
        ],
        "output": [
          {"name": "I", "value": 0},
          {"name": "J", "value": 5}
        ]
      },
      {
        "input": [
          {"name": "K", "value": 34},
          {"name": "I", "value": 1},
          {"name": "J", "value": 5}
        ],
        "output": [
          {"name": "I", "value": 1},
          {"name": "J", "value": 6}
        ]
      },
      {
        "input": [
          {"name": "K", "value": 34},
          {"name": "I", "value": 1},
          {"name": "J", "value": 10}
        ],
        "output": [
          {"name": "I", "value": 1},
          {"name": "J", "value": 10}
        ]
      },
      {
        "input": [
          {"name": "K", "value": 26},
          {"name": "I", "value": 1},
          {"name": "J", "value": 10}
        ],
        "output": [
          {"name": "I", "value": 2},
          {"name": "J", "value": 10}
        ]
      },
      {
        "input": [
          {"name": "K", "value": 26},
          {"name": "I", "value": 10},
          {"name": "J", "value": 10}
        ],
        "output": [
          {"name": "I", "value": 10},
          {"name": "J", "value": 10}
        ]
      },
      {
        "input": [
          {"name": "K", "value": 25},
          {"name": "I", "value": 10},
          {"name": "J", "value": 10}
        ],
        "output": [
          {"name": "I", "value": 10},
          {"name": "J", "value": 9}
        ]
      },
      {
        "input": [
          {"name": "K", "value": 25},
          {"name": "I", "value": 10},
          {"name": "J", "value": 0}
        ],
        "output": [
          {"name": "I", "value": 10},
          {"name": "J", "value": 0}
        ]
      }
    ]
  ]
}
```