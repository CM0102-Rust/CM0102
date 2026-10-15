# Test backlog (branches the automatic tests do not reach)

Branches marked in the Rust copy (`crate::comparacao::ramo("...")`, since batch 127) that **never ran** in the automatic careers (to 2002-06-01 with seed 12345 and, for the post-match ones, to 2003-06-01 with seed 777). Each needs a match/career that goes through it. After a session by Lucas, `python engenharia-reversa/pendencias.py` reads `jogo/ddraw.log` and says which of these ran.

Lucas's session in compare mode checks every routine listed: since 2026-10-14 the post-match ones (0x6b74a0, 0x6b6cc0, 0x6b79a0) also have a side-by-side comparison (through the tape).

| Branch | Routine | Covered in a comparing session? | Notes |
|---|---|---|---|
| `dividida vantagem` | 0x70ec80 (batch 127/126) | yes | |
| `dividida media de acertos` | 0x70ec80 | yes | |
| `dividida defesa` | 0x70ec80 | yes | |
| `disputa fora do campo` | 0x6c9cd0 (batch 127) | yes | |
| `jogada goleiro` | 0x70dfd0 (batch 129) | — | unreachable: the goalkeeper leaves at the start of the routine |
| `eventos 0x82` | 0x6b74a0 (batch 130) | yes | |
| `eventos 0x13 sem marca` | 0x6b74a0 | yes | |
| `eventos 0x13 noticia` | 0x6b74a0 | yes | |
| `jogadores marca 0x20` | 0x6b6cc0 (batch 131) | yes | runs only in the long career (seed 777, 2+) |
| `clube 5896d0` | 0x6b79a0 (batch 132) | yes | forced actions caught by the fingerprints (`leva132b.py`) |
| `clube 589bc0` | 0x6b79a0 | yes | same |
| `clube 547d90` | 0x6b79a0 | yes | same |
| `sem humano clube ligado` | 0x6b66a0 (batch 133) | yes | and the conditions always false in automatic careers: a human club in the match, competitions [0x9cf7bc]/[0x9cf6e4]/[0x9cf958], the 16th human as a national manager |
| `interesse partida marcada` | 0x6b5040 (batch 134) | yes | **covered** in Lucas's session of Oct 14 (8+, no divergence); the other humans (the 16 slots, the +0x24 link) are still pending |
| `batedor do clube` | 0x897990 (batch 138) | — | the automatic career only goes through the empty cell (16 million times): AI clubs have no chosen taker. Needs a human club with takers chosen on the tactics screen (penalty, free kick, corner) and a match with a set piece |
| `batedor de fora do clube` | 0x897990 | — | a chosen taker who is not from the club (national team or link 0x775790): a national manager with chosen takers |
| `batedor saiu do clube` | 0x897990 | — | a chosen taker who left the club (sold, loaned) before the match |
| `batedor sem ficha` | 0x897990 | — | a chosen taker without the +0x61 record (retired?) |
| `batedor alem das pessoas` | 0x897990 | — | a cell with an id beyond the people (corrupt data): probably unreachable |
| `instrucoes guardadas` | 0x896fe0 (batch 138) | — | the career only clears the instructions (32 thousand times). Saving and restoring depend on the current human ([0xb77c7a]) with the flag on: probably more than one human, or the human taking over/leaving a club |
| `instrucoes devolvidas` | 0x896fe0 | — | same |
| `instrucoes liberadas` | 0x896fe0 | — | same (without the flag) |
| `numero da competicao` | 0x6b6570 (batch 138) | — | never runs in the automatic career; called by 0x6b55c0 (match day) and by the interface: a match day with a human manager |

Batches before 127 had no marked branches: their gaps are described in each batch's text in `ROTINAS.md` (search for "lacuna", "não roda").
