# Pendências de teste (ramos que os testes automáticos não alcançam)

Ramos marcados na cópia Rust (`crate::comparacao::ramo("...")`, desde a 127ª leva) que **nunca rodaram** nas carreiras automáticas (até 2002-06-01 com a semente 12345 e, nas do pós-jogo, até 2003-06-01 com a 777). Cada um precisa de uma partida/carreira que passe por ele. Depois de uma sessão do Lucas, `python engenharia-reversa/pendencias.py` lê o `jogo/ddraw.log` e diz quais destes rodaram.

A sessão do Lucas no modo comparação confere todas as rotinas listadas: desde 14/10/2026 as do pós-jogo (0x6b74a0, 0x6b6cc0, 0x6b79a0) também têm comparação lado a lado (pela fita).

| Ramo | Rotina | Coberto na sessão comparando? | Observação |
|---|---|---|---|
| `dividida vantagem` | 0x70ec80 (127ª/126ª) | sim | |
| `dividida media de acertos` | 0x70ec80 | sim | |
| `dividida defesa` | 0x70ec80 | sim | |
| `disputa fora do campo` | 0x6c9cd0 (127ª) | sim | |
| `jogada goleiro` | 0x70dfd0 (129ª) | — | inalcançável: o goleiro sai no início da rotina |
| `eventos 0x82` | 0x6b74a0 (130ª) | sim | |
| `eventos 0x13 sem marca` | 0x6b74a0 | sim | |
| `eventos 0x13 noticia` | 0x6b74a0 | sim | |
| `jogadores marca 0x20` | 0x6b6cc0 (131ª) | sim | roda só na carreira longa (semente 777, 2+) |
| `clube 5896d0` | 0x6b79a0 (132ª) | sim | ações forçadas pegas pelas impressões (`leva132b.py`) |
| `clube 589bc0` | 0x6b79a0 | sim | idem |
| `clube 547d90` | 0x6b79a0 | sim | idem |
| `sem humano clube ligado` | 0x6b66a0 (133ª) | sim | e as condições sempre falsas nas carreiras automáticas: clube humano na partida, competições [0x9cf7bc]/[0x9cf6e4]/[0x9cf958], o 16º humano técnico de seleção |
| `interesse partida marcada` | 0x6b5040 (134ª) | sim | **coberto** na sessão do Lucas de 14/10 (8+, sem divergência); continuam pendentes os outros humanos (as 16 vagas, o vínculo +0x24) |

Levas anteriores à 127ª não tinham ramos marcados: as lacunas delas estão descritas no texto de cada leva em `ROTINAS.md` (procure "lacuna", "não roda").
