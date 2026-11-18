# 305ª leva — as viradas de temporada da Veikkausliiga, da Tippeligaen e da Allsvenskan; a virada portuguesa e a sua cadeia de acesso e rebaixamento; as viradas de temporada das copas da Finlândia, da Noruega e de Portugal; a Supertaça portuguesa: os times, a tabela da rodada e a virada

**Rotinas:**
- `fin_prm_start_new_season()` = 0x595d40 (fin_prm.cpp, thiscall, ret 0): a virada finlandesa, puxada pela Veikkausliiga: o acesso e o rebaixamento, dois passos sem nome [inferência: os grupos da Ykkönen refeitos], a Veikkausliiga refeita para o ano seguinte (regras, tabela de 12, jogos, ordem), depois a virada da Ykkönen, as posições previstas, as expectativas da diretoria e a renovação dos árbitros;
- `nor_prm_start_new_season()` = 0x7927a0 (nor_prm.cpp, thiscall, ret 0): a virada norueguesa, puxada pela Tippeligaen: o acesso e o rebaixamento, a troca de clubes entre a primeira e a segunda divisões, um passo sem nome, a Tippeligaen refeita para o ano seguinte, depois a virada da primeira divisão, as posições previstas, as expectativas e os árbitros;
- `swe_prm_start_new_season()` = 0x8905f0 (swe_prm.cpp, thiscall, ret 0): a virada sueca, puxada pela Allsvenskan: o acesso e o rebaixamento; sem a segunda divisão jogável, três clubes regionais sobem à Superettan; o nível de divisão dos clubes limitado e as seções da segunda divisão redistribuídas; a Allsvenskan refeita para o ano seguinte, depois as viradas da Superettan e da segunda divisão (quando existe), as posições previstas, as expectativas e os árbitros;
- `por_prm_process_promotion_relegation_chain()` = 0x7cfbf0 (por_prm.cpp, thiscall, ret 4): o acesso e o rebaixamento em Portugal: registra as posições finais da primeira divisão e da Liga de Honra e troca os clubes marcados entre elas; com a segunda divisão B jogada, registra as posições das suas três zonas e a Liga de Honra troca clubes com cada uma;
- `por_prm_start_new_season()` = 0x7cf9e0 (por_prm.cpp, thiscall, ret 0): a virada portuguesa, puxada pela primeira divisão: as verificações dos times B (com a segunda divisão B), o acesso e o rebaixamento, a troca da Liga de Honra com as zonas regionais (sem a segunda divisão B), a primeira divisão refeita para o ano seguinte, depois as viradas da Liga de Honra e da segunda divisão B, as posições previstas, as expectativas da diretoria e a renovação dos árbitros;
- `fin_cup_start_new_season()` = 0x592d00 (fin_cup.cpp, thiscall, ret 0): a virada da Copa da Finlândia: soltas as tabelas de inscritos, rodadas e confrontos da edição anterior, a temporada arquivada nas estatísticas e o ano + 1; depois os inscritos, as rodadas, a árvore dos confrontos e as posições previstas;
- `nor_cup_start_new_season()` = 0x78fb90 (nor_cup.cpp, thiscall, ret 0): a mesma virada na Copa da Noruega;
- `por_cup_start_new_season()` = 0x7ce540 (por_cup.cpp, thiscall, ret 0): a mesma virada na Taça de Portugal;
- `por_super_select_teams()` = 0x7d3260 (por_super.cpp, thiscall, ret 0): a Supertaça é entre o campeão da primeira divisão e o vencedor da Taça de Portugal; se o mesmo clube ganhou as duas, entra o finalista da Taça; sem um dos clubes, a caixa de erro;
- `por_super_create_round_table()` = 0x7d30f0 (por_super.cpp, thiscall, ret 0x10): a Supertaça é uma rodada: sorteio em 5 de julho e um jogo em 12 de agosto em campo neutro [inferência: o local 4] (em 2000, ida e volta nos campos dos clubes); categoria 6; sem prêmios;
- `por_super_start_new_season()` = 0x7d3490 (por_super.cpp, thiscall, ret 0): a virada da Supertaça: soltas as tabelas da edição anterior, a temporada arquivada e o ano + 1; depois os dois times, as rodadas e a árvore dos confrontos (sem posições previstas);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (11):** 10 acusadas. `mwe16_copa_portuguesa_dois_anos` escapou: ESCAPADA EXPLICADA PELA JANELA, medida: a mutacao poe a Taca de Portugal dois anos a frente na virada (`por_cup_start_new_season`, `PorCup::start_new_season` 0x7ce540). A virada roda dentro da janela (o ramo `mwe_16 portuguese cup turnover` marca 1 numa carreira Rust ate 2002-08-01, entre a virada da liga portuguesa e a da Supertaca), mas a primeira rodada da edicao seguinte cai depois de 2002-08-01: ate la a mutante gasta os mesmos sorteios da base (4857694775). Medido em 18/11/2026: base e mutante ate 2002-12-01 divergem -- base 6909527833 sorteios, 61692 partidas, impressao 81eb94f9c7dfb53d; mutante 6812199597 sorteios, 61630 partidas, impressao 316080bd39606dac. As 62 partidas a menos sao as rodadas da taca que nao aconteceram. As irmas norueguesa e finlandesa (calendario de ano civil) sao acusadas dentro da janela. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (48171c52).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 6c09145c, original bf442e79 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 304ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 305`

**Campos tocados:**
- rodada de copa +0x11 palavra sem nome = 3 (a Supertaça portuguesa); valor; grava
