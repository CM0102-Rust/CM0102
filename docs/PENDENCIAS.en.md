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
| `simulacao rapida abandono` | 0x6beeb0 (batch 140) | — | more than 4 players sent off on one side in the quick simulation (3-0 to the other): never happens in the automatic career up to 2002-06-01 (sendings-off 2048 times, second yellow 1024). Maybe unreachable in practice; would need a side with many cards |
| `simulacao rapida contador reiniciado` | 0x6beeb0 | — | at the end of the quick simulation, the attack counter (+0xf5e2/+0xf618) below 10 restarts (20 and draw(6) + 12): attacks exceed 10 in every match of the career |
| `lesao para 3 e sai` | 0x6d0980 (batch 141) | — | a light injury (severity up to 50) with the stamina taken above the limit (0x96d1a8) on a player already marked at +0x1c (an earlier serious injury in the same match): never happens in the automatic career up to 2002-06-01 |
| `chute a gol marca` | 0x70a910 (batch 142) | — | the career milestone commentaries (first goal, 50th, 100th...; 0x200a..0x2012, highlight 0x21b6) never come up in the automatic career up to 2002-06-01: goals in detailed matches are few |
| `chute a gol empate tardio` | 0x70a910 | — | an equaliser after 70 minutes in a detailed match (0x1fd6..0x1fd9): does not happen in the automatic career |
| `movimento sem vaga na grade` | 0x6f09f0 (batch 143) | — | a player moving with no slot in the cell (+0x28 negative) enters the new cell; in the automatic career everyone already takes the field with a slot |
| `movimento sem mais adiantado` | 0x6f09f0 | — | side 1 with no furthest-forward player recorded (+0xf59a empty) uses row 9; in the automatic career the record is never empty when someone moves |
| `conducao goleiro sem marca` | 0x7109f0 (batch 144) | — | the opponent's position-1 guard without the goalkeeper mark (table &1 = 0) gives +5000 to the shot score; in the automatic career the position-1 guard is always a goalkeeper |
| `laco ficha sem motor` | 0x6b98c0 (batch 145) | — | an on-field record (+0x19 >= 0) with no engine (+0x19e = 0) is removed (-5) at the start of the loop; does not happen in the automatic career |
| `laco cobrador da bola parada` | 0x6b98c0 | — | a set piece with no ball owner (or a sent-off one) picks the taker (0x6ceb90) and turns him to goal; the restart already leaves an owner in the automatic career |
| `chute bloqueado` | 0x702050 (batch 147) | — | the interception on the way to goal returns 2 (someone deflects): 0x1fd3/0x1fd4, the shot changes by draw(1000) - draw(2000) and, off target, goes out for a corner; it did not happen in the automatic career up to 2002-06-01 |
| `prejogo ceu com chuva` | 0x6b7f20 (batch 148) | — | with the rain flag (+0x4d & 0x2000) and the sky outside 0/2/4, the loop's limit +0xf638 comes from draw(25) + 15 or draw(30) + 50; in the automatic career up to 2002-06-01 the limit never mattered (the mutation that fixes it at 1 changes nothing) |
| `prejogo codigo da insatisfacao` | 0x6b7f20 | — | the pre-match event 0x13 (player unhappy with his contract) goes into the list with the code; in the automatic career whoever reads it does not change the result (the mutation to 0x1e changes nothing) |
| `escalacao ficha do tecnico` | 0x6db500 (batch 149) | — | the tactics copy three fields of the manager's record (+0xa, +8, +0xc) to +0, +2, +4; in the automatic career the second (+2) does not matter (the mutation that swaps it changes nothing) |
| `dia passo a passo` | 0x6b5ce0 (batch 151) | — | one engine step per call (without `ate_o_fim`) does not run in the automatic career: the branch counter "andar um passo" stays at zero; there the day loop plays each match to the end |
| `dia contador de terminadas` | 0x6b5ce0 (batch 151) | — | the day's +0x25 (finished matches) only feeds the bar's percentage; counting 2 instead of 1 does not change the career |
| `dia rodada do humano` | 0x6b5f00 (batch 151) | — | the round (+0x2d from 0x4a4850) and the swap of sides in 0x6b79a0 only exist with a human manager; the counter "fechar rodada do humano" stays at zero in the automatic career |
| `dia historia` | 0x6b5f00 (batch 151) | — | [0xb74434]'s method 1 (the history) runs (counter "fechar historia" above 2048), but the history is not part of the career fingerprints (draws, matches, database): removing the call changes nothing in them |
| `dia linhas do humano` | 0x6b4360 (batch 152) | — | at the end of the day, the humans' stored tactics (0x5fbe20, 0x5fbd00, 0x5b5960) are only redone with a human manager; the counter "linhas do humano refeitas" stays at zero in the automatic career (the mutation that removes the call escapes; the finished counter, +0x25, is the gap already noted in batch 151) |
| `dia primeira volta` | 0x6b3c10 (batch 153) | — | the day's 1st round (+0x33 = 1) only starts matches whose competition (the match's +0x63) has +0x10, which the setup creates zeroed; in the automatic career the 1st round picks nothing and everything falls to the 2nd, so starting in the 2nd or pointing to another competition does not change the career |
| `dia partidas ja marcadas` | 0x6b3c10 (batch 153) | — | the setup skips the list's matches already marked -2; in the automatic career none arrives marked like that (the mutation that lets them in escapes, even the strong one) |
| `sons dos eventos` | 0x6d8c70 (batch 153) | — | waiting for the event sounds only runs with the match screen (0x6b55c0, network game): the counter "sons dos eventos" stays at zero in the automatic career |

Batches before 127 had no marked branches: their gaps are described in each batch's text in `ROTINAS.md` (search for "lacuna", "não roda").
