# 8-league audit (night of 2 to 3 Nov 2026)

Leagues: `ENG ESP ITA FRA BRA JPN AUS USA`; main DLL with the 271st batch published (1,646 routines).

## Careers to 2003-01-31 (bench 2)
- Rust ×2: equal (fingerprint `d4d55f23260a3cd7`, 99,629 matches), garbage detector **equal**, determinism OK.
- Original (recorded only): `b719367c681948a9`, 99,265 matches.

## Comparing to 2002-06-01 (bench 3)
Date 0 and random 0 divergences; engine: **20 `DIVERGENCIA` lines** (the summary counter shows 17; it is only printed at milestones).

| Routine | Lines | When | Batch |
|---|---|---|---|
| `campeao_da_liga_nos_registros` (0x868c50) | 17 | May 2002 | 267th (cqd_13) |
| `campeao_da_copa_nos_registros` | 1 | 2001-306 | 267th (cqd_13) |
| `posicoes_e_situacao_na_liga` | 1 | 2001-325 | 262nd |
| `renovar_arbitros` | 1 | 2002-011 | 261st |

No game message boxes. All 4 routines pass the chain (ENG only, to 2002-01-01): the chain reaches neither May nor the other leagues.

## Fixes
- cqd_13: being fixed by an agent (own copy); the patch goes in once it is proven with the ENG comparison to 2002-06-01.
- cqd_13: the comparator did not redo the morale draw or the scratch date 0xae2470 after the calls the dry run skips (renewal, call-ups, news); the Rust routine was right.
- 261st `renovar_arbitros`: port defect (it cleared the stats row of the first referee in the range only; the original clears each one's).
- 262nd `posicoes_e_situacao_na_liga`: comparator defect (the dry-run copy lost what the announcement writes to the table in the Brazil, Belgium and Finland classes; now the original records a tape and the copy replays it).
- cpx_1 `retirar_oferta_de_transferencia`: comparator defect (the dry run skipped 0x4cf4a0, which returns the stored contract offers and moves the rival offer from 0xf to 8, and 0x8cbf70 in state 0x13, which clears bit 0x10 of the person's offer mark; the dry run now reproduces both effects and undoes them before the original). Comparing 8 leagues to 2002-031 with no divergence.

## Comparing `ENG SCO IRL ITA ESP` to 2002-06-01 (bench 3)
Reached 2002-151; date 0, random 0; only cqd_13 lines (14 league, 1 cup). `retirar_oferta` (cpx_1) and `aviso_ao_rival_de_posicao` (cqb_8), seen before with these leagues, did not diverge.
