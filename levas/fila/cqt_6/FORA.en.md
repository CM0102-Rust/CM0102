# cqt_6 without PRONTO (06/11/2026)

The three Brazilian Champions Cup routines (0x426130 preliminary table, 0x427320 preliminary stage, 0x4263c0 next
stage) run in the 8-league career (2001-167, 2001-183 and 2001-200; the mutation counters ran) and show no divergence
in the comparison up to 2001-10-01 (the next-stage plan was relaxed: the groups write their own stage into +0x30
afterwards; the long comparison up to 2002-03-15 flagged that for nothing). The two Rust careers with the detector are
equal. But the mutations do not change the career result:

- the 3 preliminary clubs staying in the entrants list (+0x14) escaped: the groups do not take teams from that list
  [inference];
- the preliminary table without its initial order (League::sort_table) escaped: the league reorders later;
- the preliminary rows in reverse order crash the game (EXCECAO in the DLL on 2001-167): caught only by the crash.

This is the 279th–281st lesson (the Champions Cup is a passing tournament: nothing read later depends on it inside the
window). Ways out: go in as passengers in an item with an effective routine, or target the champion (the Libertadores
place), decided in the semi-finals and the final (0x426720 and the final, not ported yet).
