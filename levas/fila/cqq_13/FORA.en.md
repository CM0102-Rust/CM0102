# cqq_13 without PRONTO (06/11/2026)

The three routines (the clubs' records in the Coppa Italia, the Serie C Cup and the LDV Vans Trophy: 0x63ff00,
0x63e270, 0x56bee0) run in the 8-league career (78, 142 and 110 calls up to 2002-06-01; the mutation counters ran)
and show no divergence in the comparison up to 2001-10-01. But the mutation does not change the career's result:

- the group-qualification weight in the Coppa Italia (0x28 → 0x14) escaped: the Rust career up to 2002-06-01 came out
  equal to the base (fingerprint f29c85195c2a534f);
- the other two (weight zero in the Serie C Cup, minimum elimination weight in the LDV) were not run to the end (the
  run was stopped so as not to hold the laboratory lock); by the same mechanism they should escape.

The staff records (0x86bdd0, 0x86c000, 0x86c1d0, 0x86a800, 0x86b0b0) go to the history; within the 17-month window
nothing the career reads depends on the weight [inference]. The caught mutations of cqi_10 and cpz_4 in these routines
touched the club's European competition mark (+0x1db), which these cups do not write.

Ways out: enter as passengers in an item with a routine that has an effect, or wait for a history mutation the
career reads (for example the manager's reputation at the season turnover, after 2002-06-01).
