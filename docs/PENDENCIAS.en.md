# Test backlog (branches the automatic tests do not reach)

Branches marked in the Rust copy (`crate::comparacao::ramo("...")`, since batch 127) that **never ran** in the automatic careers (to 2002-06-01 with seed 12345 and, for the post-match ones, to 2003-06-01 with seed 777). Each needs a match/career that goes through it. After a session by Lucas, `python engenharia-reversa/pendencias.py` reads `jogo/ddraw.log` and says which of these ran.

**Note:** Lucas's session in compare mode only checks routines **with a side-by-side comparison**. The post-match ones (0x6b74a0, 0x6b6cc0, 0x6b79a0) stay original in that mode: the session tests nothing for them (nor counts their branches) until they get a comparison of their own.

| Branch | Routine | Covered in a comparing session? | Notes |
|---|---|---|---|
| `dividida vantagem` | 0x70ec80 (batch 127/126) | yes | |
| `dividida media de acertos` | 0x70ec80 | yes | |
| `dividida defesa` | 0x70ec80 | yes | |
| `disputa fora do campo` | 0x6c9cd0 (batch 127) | yes | |
| `jogada goleiro` | 0x70dfd0 (batch 129) | — | unreachable: the goalkeeper leaves at the start of the routine |
| `eventos 0x82` | 0x6b74a0 (batch 130) | **no** (no comparison) | |
| `eventos 0x13 sem marca` | 0x6b74a0 | **no** | |
| `eventos 0x13 noticia` | 0x6b74a0 | **no** | |
| `jogadores marca 0x20` | 0x6b6cc0 (batch 131) | **no** | runs only in the long career (seed 777, 2+) |
| `clube 5896d0` | 0x6b79a0 (batch 132) | **no** | forced actions caught by the fingerprints (`leva132b.py`) |
| `clube 589bc0` | 0x6b79a0 | **no** | same |
| `clube 547d90` | 0x6b79a0 | **no** | same |

Batches before 127 had no marked branches: their gaps are described in each batch's text in `ROTINAS.md` (search for "lacuna", "não roda").
