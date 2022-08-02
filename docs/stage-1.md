# Stage 1

#### 1-3
See `src/main/com/example/calculator/calculator.kt`.

#### 4
Adding unit tests, with data known to be correct, should check the calculations are correct.

Possible edge cases are angles beyond the `(0, 0.5PI)` range, or a launch velocity of `0`.
They should be included as unit test cases, and/or input validators/guards should be added.

#### 5
Wind resistance is a major factor excluded from the physics model implemented;
including it would significantly raise the system's accuracy.

