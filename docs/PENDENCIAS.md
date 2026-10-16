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
| `batedor do clube` | 0x897990 (138ª) | — | a carreira automática só passa pela célula vazia (16 milhões de vezes): os clubes da IA não têm batedor escolhido. Precisa de um clube humano com batedores escolhidos na tela de táticas (pênalti, falta, escanteio) e uma partida com bola parada |
| `batedor de fora do clube` | 0x897990 | — | batedor escolhido que não é do clube (seleção ou vínculo 0x775790): técnico de seleção com batedores escolhidos |
| `batedor saiu do clube` | 0x897990 | — | batedor escolhido que saiu do clube (vendido, emprestado) antes da partida |
| `batedor sem ficha` | 0x897990 | — | batedor escolhido sem a ficha +0x61 (aposentado?) |
| `batedor alem das pessoas` | 0x897990 | — | célula com id acima das pessoas (dado corrompido): provavelmente inalcançável |
| `instrucoes guardadas` | 0x896fe0 (138ª) | — | a carreira só apaga as instruções (32 mil vezes). Guardar e devolver dependem do humano da vez ([0xb77c7a]) com o flag ligado: provavelmente mais de um humano, ou o humano assumindo/deixando um clube |
| `instrucoes devolvidas` | 0x896fe0 | — | idem |
| `instrucoes liberadas` | 0x896fe0 | — | idem (sem o flag) |
| `numero da competicao` | 0x6b6570 (138ª) | — | nunca roda na carreira automática; chamada pela 0x6b55c0 (dia de jogos) e pela interface: dia de jogo com técnico humano |
| `simulacao rapida abandono` | 0x6beeb0 (140ª) | — | mais de 4 expulsos de um lado na simulação rápida (3 x 0 para o outro): nunca acontece na carreira automática até 2002-06-01 (expulsões 2048 vezes, segundo amarelo 1024). Talvez inalcançável na prática; precisaria de um lado com muitos cartões |
| `simulacao rapida contador reiniciado` | 0x6beeb0 | — | no fim da simulação rápida, o contador de ataques (+0xf5e2/+0xf618) abaixo de 10 reinicia (20 e sorteio(6) + 12): os ataques passam de 10 em toda partida da carreira |

Levas anteriores à 127ª não tinham ramos marcados: as lacunas delas estão descritas no texto de cada leva em `ROTINAS.md` (procure "lacuna", "não roda").
