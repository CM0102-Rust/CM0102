# Performance: measured baselines

Measured with the working DLL (detours into the original exe), to compare against a future standalone executable.
Always on the same bench with no other tests running; every number was repeated and came out the same.

## Creating a new game (from process start to the career's first day)

`simular.py --ate 2001-07-25 --motor sim --modo {rust|original} --ligas {ENG|MUNDO}`, bench 5, 15/11/2026,
with 2,478 routines in Rust (batch 299 published).

| Leagues | Original | Routines in Rust (DLL) | Difference |
|---|---|---|---|
| ENG | 9 s | 10 s | +11% |
| MUNDO (209 nations) | 238 s | 253 s | +6% |

**Reading:** the Rust copies are currently *slower* than the original, and the cause is the shape of the work, not
the copies: every ported routine is reached through a detour (one extra jump and the DLL's private stack), and
creating the world calls the competition constructors thousands of times. In a single executable, with no DLL and
no trampoline, that layer goes away.

**Where the world-creation time goes** (hypotheses to measure, not measured yet): reading the database in many small
calls (the same pattern we found in the save container), heavy allocation (one competition at a time, on the
period's library allocator) and the fixed-size loops the rewrite keeps finding.

**On 64-bit:** nothing in this measurement depends on pointer size. The 64-bit obstacle is a different one: the
original computes on the x87 with 80-bit intermediates, which SSE does not have; a 64-bit executable would change
results (reputation, morale, value) unless the precision is emulated, which costs the very performance one wanted.
