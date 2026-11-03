# Auditoria das 8 ligas (madrugada de 02 para 03/11/2026)

Ligas: `ENG ESP ITA FRA BRA JPN AUS USA`; DLL da principal com a 271ª publicada (1.646 rotinas).

## Carreiras até 2003-01-31 (bancada 2)
- Rust ×2: iguais (impressão `d4d55f23260a3cd7`, partidas 99.629), detector de lixo **igual**, determinismo OK.
- Original (só registrada): `b719367c681948a9`, partidas 99.265.

## Comparando até 2002-06-01 (bancada 3)
Data 0 e aleatório 0 divergências; motor: **20 linhas `DIVERGENCIA`** (o contador do resumo marca 17; ele só é impresso nos marcos).

| Rotina | Linhas | Quando | Leva |
|---|---|---|---|
| `campeao_da_liga_nos_registros` (0x868c50) | 17 | maio de 2002 | 267ª (cqd_13) |
| `campeao_da_copa_nos_registros` | 1 | 2001-306 | 267ª (cqd_13) |
| `posicoes_e_situacao_na_liga` | 1 | 2001-325 | 262ª |
| `renovar_arbitros` | 1 | 2002-011 | 261ª |

Nenhuma caixa do jogo. As 4 rotinas passam na cadeia (só ENG até 2002-01-01): a cadeia não chega a maio nem às outras ligas.

## Correções
- cqd_13: em correção por um agente (cópia própria); o patch entra quando ele provar com o comparando ENG até 2002-06-01.
- cqd_13: o comparador não refazia o sorteio do moral nem a data de rascunho 0xae2470 depois das chamadas que o ensaio pula (renovação, convocações, notícia); a rotina Rust estava certa.
- 261ª `renovar_arbitros`: defeito do porte (zerava a linha de estatísticas só do primeiro árbitro da faixa; a original zera a de cada um).
- 262ª `posicoes_e_situacao_na_liga`: defeito do comparador (a cópia em ensaio perdia o que o anúncio grava na tabela nas classes do Brasil, Bélgica e Finlândia; agora a original grava numa fita e a cópia repõe).
- cpx_1 `retirar_oferta_de_transferencia`: defeito do comparador (o ensaio pulava a 0x4cf4a0, que devolve as ofertas de contrato guardadas e passa a oferta rival de 0xf para 8, e a 0x8cbf70 do estado 0x13, que apaga o bit 0x10 da marca de oferta da pessoa; agora o ensaio reproduz os dois efeitos e desfaz antes da original). Comparando 8 ligas até 2002-031 sem divergência.

## Comparando `ENG SCO IRL ITA ESP` até 2002-06-01 (bancada 3)
Chegou a 2002-151; data 0, aleatório 0; só linhas da cqd_13 (14 da liga, 1 da copa). A `retirar_oferta` (cpx_1) e o `aviso_ao_rival_de_posicao` (cqb_8), vistos antes com essas ligas, não divergiram.
