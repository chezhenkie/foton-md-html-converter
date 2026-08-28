# Three Complex Diagrams Demo

## Diagram 1: Microservices Architecture

```svg
 +--------+
 | Gateway|
 +---+----+
    /|\
   / | \
  v  v   v
Auth Users Orders
  |   |    |
  v   v    v
Cache DB  Queue
  \   |    /
   \  |   /
    v v  v
    +------
    | Merge|
    +---+--+
        |
        v
      Logger
```

## Diagram 2: CI/CD Pipeline with Branching

```svg
Source ---> Build ---> Test ---> Stage ---> Deploy
                     /    \                    ^
                    v      v                   |
                  Lint    Unit            (fail loop)
                    \      /
                     v    v
                    Report
```

## Diagram 3: State Machine

```svg
 +------+      +---------+      +--------+
 | Idle | ---> | Pending | ---> | Active |
 +------+      +----+----+      +---+----+
     ^              |               |
     |              v               v
     |         +-----------+    +-------+
     +--------+ Cancelled |    | Error |
     ^        +-----------+    +---+---+
     |                             |
     +----------(retry)------------+
```
