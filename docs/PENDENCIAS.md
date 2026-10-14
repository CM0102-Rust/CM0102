# Pendências de teste (ramos que os testes automáticos não alcançam)

Ramos marcados na cópia Rust (`crate::comparacao::ramo("...")`, desde a 127ª leva) que **nunca rodaram** nas carreiras automáticas (até 2002-06-01 com a semente 12345 e, nas do pós-jogo, até 2003-06-01 com a 777). Cada um precisa de uma partida/carreira que passe por ele. Depois de uma sessão do Lucas, `python engenharia-reversa/pendencias.py` lê o `jogo/ddraw.log` e diz quais destes rodaram.

**Atenção:** a sessão do Lucas no modo comparação só confere as rotinas **com comparação lado a lado**. As do pós-jogo (0x6b74a0, 0x6b6cc0, 0x6b79a0) ficam na original nesse modo: para elas a sessão não testa nada (nem conta os ramos) até que ganhem uma comparação própria.

| Ramo | Rotina | Coberto na sessão comparando? | Observação |
|---|---|---|---|
| `dividida vantagem` | 0x70ec80 (127ª/126ª) | sim | |
| `dividida media de acertos` | 0x70ec80 | sim | |
| `dividida defesa` | 0x70ec80 | sim | |
| `disputa fora do campo` | 0x6c9cd0 (127ª) | sim | |
| `jogada goleiro` | 0x70dfd0 (129ª) | — | inalcançável: o goleiro sai no início da rotina |
| `eventos 0x82` | 0x6b74a0 (130ª) | **não** (sem comparação) | |
| `eventos 0x13 sem marca` | 0x6b74a0 | **não** | |
| `eventos 0x13 noticia` | 0x6b74a0 | **não** | |
| `jogadores marca 0x20` | 0x6b6cc0 (131ª) | **não** | roda só na carreira longa (semente 777, 2+) |
| `clube 5896d0` | 0x6b79a0 (132ª) | **não** | ações forçadas pegas pelas impressões (`leva132b.py`) |
| `clube 589bc0` | 0x6b79a0 | **não** | idem |
| `clube 547d90` | 0x6b79a0 | **não** | idem |

Levas anteriores à 127ª não tinham ramos marcados: as lacunas delas estão descritas no texto de cada leva em `ROTINAS.md` (procure "lacuna", "não roda").
