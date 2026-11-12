# nma_1: game_daily_staff_update (0x5ce2d0) fica fora desta rodada

*Versão em inglês: [FORA.en.md](FORA.en.md).*

A rotina `game_daily_staff_update` (0x5ce2d0..0x5d5a1f, 30.544 bytes, game.cpp, cdecl(void)) não foi portada nesta
sessão (12/11/2026, portador `nmaA`). Nada dela entrou na fila: uma cópia parcial não pode ser desviada (a rotina é
um desvio só) e o item só vale inteiro.

## Por que

- **Tamanho:** 9.229 instruções, 866 chamadas (395 `utils_random`, 54 `_ftol` 0x945640, 8 0x945ff0), quadro de 0x888
  bytes com um `LLIST` local (0x68d020/0x68d040) e um `qsort` (0x945333) no meio. É o corpo por pessoa de um laço sobre
  toda a tabela de pessoas (`G_STAFF_TABLE`, 0x6e bytes, até `G_STAFF_COUNT`), mais o fecho (ordenação de 0xae2d24 por
  0x5d6220 e a cópia de 0xae2d20 para +5 com teto em +7, só se alguém marcou o byte local). Portar fiel byte a byte
  (ordem do x87, os `_ftol`, cada sorteio na ordem) dá na casa de 3.000 linhas de Rust; não cabe numa sessão com a
  fidelidade que a fila exige.
- **Prova:** a rotina não pode rodar duas vezes (mexe em todas as pessoas, contratos, disputas, notícias, mídia,
  listas), então o comparador seria "só a original", como o das outras passadas do dia. Com o critério de 31/10 (as
  carreiras Rust ×2 comparadas entre si, a original só registrada), um erro de porte não aparece em nenhum teste: só as
  mutações a cobririam. O comparador por pedaço pedido não tem como rodar a original pedaço a pedaço (o corpo por
  pessoa não é uma função). Antes de portar, vale decidir com o Lucas como provar: por exemplo, um comparador que
  fotografa os campos da própria pessoa e do contrato dela, roda a original numa tabela de uma pessoa só (trocando
  `G_STAFF_TABLE`/`G_STAFF_COUNT` por uma tabela de 1) e depois a cópia, com `STATE_REGIONS` e as notícias contadas, e
  nomeia o pedaço que divergiu pelos `covered!`; isso precisa ser provado viável antes (as rotinas chamadas acham a
  pessoa pelo id na tabela).

## Mapa dos pedaços (para a próxima rodada)

Fronteiras onde nenhum salto curto (até 3000 bytes) atravessa (script `mapa_pedacos.py` nesta pasta). Os
saltos longos que atravessam vários pedaços: 0x5ce400→0x5d3367 (quem não é jogador pula para a parte da comissão),
0x5cfa53/5e/85/91→0x5d2392, 0x5cfa91→0x5d352d, 0x5d111e..0x5d1155→0x5d28f8, 0x5d3938/0x5d3a8d→0x5d578d.

| pedaço | faixa | bytes | o que chama (resumo) |
|---|---|---|---|
| entrada | 0x5ce33c–0x5ce4b6 | 378 | filtro (+0x1a; general.dat +0xb & 0x30 = 0x20 pula), contrato (0x4cf5a0 → 0xae1080), time B, recupera fadiga, contrato no clube |
| colega preferido | 0x5ce4b6–0x5ce5e7 | 305 | `staff_pick_favoured_team_mate`, fadiga, sorteios, clube humano |
| forma e lista | 0x5ce5e7–0x5ce9b9 | 978 | forma recente, na lista de transferência |
| moral | 0x5ce9b9–0x5cf106 | 1.869 | 77 sorteios, bandeiras do contrato, transferência futura |
| antipatias e treino | 0x5cf106–0x5cf391 | 651 | estatísticas, antipatia, lesão, AWOL, nota da comissão, relatório de treino |
| disputas | 0x5cf391–0x5cf4c8 | 311 | `DISPUTE::*`, notícia de disputa, quer sair |
| queixas do contrato | 0x5cf4c8–0x5cfa64 | 1.436 | Comp_Stats, simpatias do clube, moral, bandeiras e queixas, `Date::days_since` |
| lesão e listas | 0x5cfa64–0x5cfc42 | 478 | lesão, listas dos clubes, finanças, eventos do contrato |
| queixas 2 | 0x5cfc42–0x5cfd53 | 273 | queixas, bandeiras |
| história na mídia | 0x5cfd53–0x5cff5e | 523 | `Media::create_player_media_story` |
| desempenho | 0x5cff5e–0x5d0e27 | 3.785 | 11 estatísticas, nota, aparições, elogio na mídia, seleção, agradece ao técnico |
| valor e simpatias | 0x5d0e27–0x5d10e5 | 702 | quer sair, valor do jogador, simpatias por clubes |
| aposentadoria e livre | 0x5d10e5–0x5d1327 | 578 | faixa de riqueza, aposentadoria, transferência livre, pede garantias |
| contrato | 0x5d1327–0x5d29de | 5.815 | 33 sorteios, ofertas de contrato, quer novo contrato, status merecido, empréstimo, ofertas |
| antipatias da comissão | 0x5d29de–0x5d2d9f | 961 | antipatias, contratos, suspensão, status merecido |
| queixas 3 | 0x5d2d9f–0x5d2fb3 | 532 | queixas, treino de posição |
| idiomas e ausência | 0x5d2fb3–0x5d377e | 1.995 | idiomas, eventos, AWOL, normaliza a comissão |
| condição física | 0x5d377e–0x5d38f9 | 379 | treino, lesão, condição de jogo, seleção, má forma |
| ordenação | 0x5d38f9–0x5d3a13 | 282 | 0x944e46 ×2, `qsort` |
| simpatias por clubes | 0x5d3a13–0x5d3d09 | 758 | nota média, caixa de erro, simpatias por clubes |
| atributos da comissão (5 pedaços) | 0x5d3d09–0x5d4320 | 1.559 | sorteios, `pow_double`, estatísticas, fadiga |
| lesão e atributos | 0x5d4320–0x5d4800 | 1.248 | lesão, clube humano, notas da comissão |
| fadiga | 0x5d4800–0x5d4996 | 406 | fadiga |
| tempo de jogo | 0x5d4996–0x5d4ad2 | 316 | humor de tempo de jogo, rodízio, nota para escalação, relatório de treino |
| atributos do jogador (2 pedaços) | 0x5d4ad2–0x5d5771 | 3.231 | 91 sorteios, `clamp_char_sum` ×26, treino |
| fecho da pessoa | 0x5d5771–0x5d5864 | 243 | normaliza, apaga relações, listas dos clubes |

Os nomes dos pedaços saem das chamadas, não de leitura do código [inferência]. A desmontagem inteira:
`python engenharia-reversa/desmonta.py jogo/cm0102.exe 0x5ce2d0 0x5d5ae0`. A rotina é igual no cm0102-1080.exe (os
30.544 bytes conferidos). Na carreira ENG até 2002-06-01 ela roda 322 vezes (`docs/cobertura_simulacao.txt`).
