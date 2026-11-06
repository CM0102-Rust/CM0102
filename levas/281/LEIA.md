# 281ª leva — o fim da temporada das ligas: a cadeia de acessos e rebaixamentos; as notícias do prêmio e da TV de uma competição; o calendário de mais copas de clubes: a Copa dos Campeões, a Merconorte e as Supercopas da Europa e da Ásia; o calendário das copas de clubes da Ásia e do Troféu da Liga inglesa: a Liga dos Campeões e a Recopa asiáticas e o LDV Vans Trophy; o calendário dos torneios de seleções: a Copa América, a Copa Africana de Nações e a Copa das Confederações; o calendário das copas intercontinentais e da Intertoto: a Intertoto, a Copa Intercontinental, a Copa Interamericana e a Copa dos Campeões da CONCACAF; a Copa da UEFA e a Intertoto: as rodadas da Copa da UEFA, os inscritos da Intertoto e a lista dos seus clubes; os clubes das supercopas: a Supercopa Europeia, a Supercopa Asiática e a Supercopa da Série C1; as copas de clubes da Oceania e da Ásia: o calendário do Campeonato de Clubes da Oceania e os times da Liga dos Campeões e da Recopa asiáticas

**Rotinas:**
- `cadeia_de_acessos()` = 0x689c20 (league.cpp, thiscall, ret 4): no fim da temporada, só a liga do topo da pirâmide (sem liga acima) fecha a classificação e desce a cadeia das divisões: em cada par (a de cima e a de baixo), fecha a de baixo e troca os que sobem e os que descem (0x689c80, com as cláusulas de contrato, aos pares ou em bloco, sem conferir quantos);
- `noticia_do_premio()` = 0x48e3e0 (comp.cpp, thiscall, ret 0x1c): a notícia do prêmio em dinheiro ao técnico humano do clube, só quando o prêmio passa de 50 × a reputação do clube: a competição, o valor, a posição, a rodada, o tipo e, com a partida, o adversário e se o clube venceu;
- `noticia_da_tv()` = 0x48ea40 (comp.cpp, thiscall, ret 8): a notícia dos direitos de TV ao técnico humano do clube: o clube, a competição, o valor e o ano;
- `cqp10_copa_dos_campeoes_tabela()` = 0x425c10 (bra_champ_cup.cpp, thiscall, ret 0x10): regra: a Copa dos Campeões tem uma preliminar (fase −1, id 0xe6) de três rodadas às segundas [inferência: dia da semana 1], 19 e 26 de junho e 3 de julho; depois dois grupos (fases 0 e 1, id 0x3fd + fase) de três rodadas, 9, 16 e 20 de julho; o mata-mata (fase 2 em diante): semifinal sorteada em 21/7 e jogada em 23/7, volta 2 dias depois, e final sorteio 26/7, jogo em 27/7, volta 3 dias depois; categoria 6; desempate de ida e volta 1; prêmio da final 800 mil ao campeão e 400 mil ao vice;
- `cqp10_merconorte_tabela()` = 0x4c1960 (con_merc_cup.cpp, thiscall, ret 0x10): regra: a Merconorte começa por grupos (fases abaixo de 3, id 0x3e9 + fase: 4 grupos [inferência]) de seis rodadas às terças de 15 em 15 dias, de 9/9 a 18/11; o mata-mata (fase 3): semifinal sorteada em 19/11 e jogada em 25/11 em ida e volta (7 dias), final sorteio 3/12 e jogo único em 9/12; categoria 8; desempate 3;
- `cqp10_supercopa_europeia_tabela()` = 0x57a3c0 (eur_super_cup.cpp, thiscall, ret 0x10): regra: a Supercopa Europeia é um jogo único em 24 de agosto (sorteio em 4 de julho) em campo neutro (mando 0x8000: Mônaco [inferência]), categoria 8, desempate 0x83, sem prêmios;
- `cqp10_supercopa_asiatica_tabela()` = 0x410910 (asia_super_cup.cpp, thiscall, ret 0x10): regra: a Supercopa Asiática é em ida e volta, o primeiro jogo em 4 de agosto (sorteio em 4 de julho) e a volta 14 dias depois, categoria 6, desempate de ida e volta 0x83, sem prêmios;
- `cqp11_asia_clubes_tabela()` = 0x40ae90 (asia_club_champ.cpp, thiscall, ret 0x10): regra: a Liga dos Campeões asiática tem, em cada zona (fases −1 e 0, ids 0x480 e 0x481), duas eliminatórias em ida e volta (1ª sorteada em 1/8 e jogada em 23/8, 2ª em 14/9 e 20/9; 21 dias entre os jogos na 1ª, 7 na 2ª); os grupos finais (fases 1 e 2, ids 0x482 e 0x483) de três rodadas em 21, 23 e 25 de março do ano seguinte com os confrontos fixos (os jogos 0 e 1 de cada rodada); a fase final (fase 3) em maio: semifinal em 24/5, terceiro lugar e final em 26/5, em jogo único (desempate 0x383 e 0x83); categoria 4 nas eliminatórias e 6 depois;
- `cqp11_asia_recopa_tabela()` = 0x40cf90 (asia_cup_winner.cpp, thiscall, ret 0x10): regra: a Recopa asiática tem, em cada zona (fases −1 e 0, ids 0x480 e 0x481), três eliminatórias em ida e volta: 1ª em 23/8, 2ª em 20/9, quartas em 24 de março do ano seguinte (21 dias entre os jogos na 1ª, 7 depois); a fase final (fase 1) em maio: semifinal em 24/5, terceiro lugar e final em 26/5, em jogo único; categoria 4 nas eliminatórias e 6 depois;
- `cqp11_trofeu_tabela()` = 0x56ade0 (eng_auto_cup.cpp, thiscall, ret 0x10): regra: o LDV Vans Trophy tem duas chaves (fases −1 e 0, Norte e Sul [inferência], ids 0x41d e 0x41e) de cinco rodadas em jogo único: 1ª em 10/12 (24 times), 2ª em 14/1 (16 times, 4 novos), quartas em 4/2, semifinal em 18/2 e a final da chave em ida e volta a partir de 4/3 (7 dias); a final (fase 1) sorteada em 12/3 e jogada em 20/4 do ano seguinte com mando 4 (estádio neutro [inferência]); categoria 4 nas chaves e 6 nas finais; sem prêmios por rodada;
- `cqp12_copa_america_tabela()` = 0x5e0900 (goldcup.cpp, thiscall, ret 0x10): regra: a Copa América tem três grupos (fases −1, 0 e 1, ids 0x41f a 0x421) de três rodadas em 6, 10 e 13 de julho, cada rodada com os dois jogos fixos do grupo (os confrontos e o mando de cada um); o mata-mata (fase 2) em jogo único: quartas em 16/7 com os confrontos fixos (os dois melhores terceiros [inferência]), semifinal em 19/7, terceiro lugar em 22/7 e final em 23/7 (mando 4, campo neutro [inferência]); categoria 10; desempate 0x103 até a semifinal e 3 na final; sem prêmios;
- `cqp12_africana_tabela()` = 0x4012e0 (african_nations.cpp, thiscall, ret 0x10): regra: a Copa Africana de Nações tem quatro grupos (fases −1 a 2, ids 0x3e8 a 0x3eb) de três rodadas em 8, 12 e 16 de fevereiro, com os jogos fixos de cada grupo; o mata-mata (fase 3) em jogo único: quartas em 21/2 com os confrontos fixos, semifinal em 25/2, terceiro lugar em 27/2 e final em 28/2 (mando 4); categoria 10; desempate 0x303 até a semifinal e 0x203 na final; sem prêmios;
- `cqp12_confed_tabela()` = 0x58b560 (fifa_confed.cpp, thiscall, ret 0x10): regra: a Copa das Confederações tem dois grupos (fases −1 e 0, ids 0x3e8 e 0x3e9) de três rodadas em 29 de julho, 1º e 4 de agosto, com os jogos fixos; o mata-mata (fase 1) em jogo único: semifinal em 6/8, terceiro lugar em 8/8 e final em 9/8 (mando 4); categoria 10; desempate 0x383 na semifinal e 0x83 depois;
- `cqp13_intertoto_tabela()` = 0x633300 (intertoto_cup.cpp, thiscall, ret 0x10): regra: a Intertoto tem 5 rodadas em ida e volta: 1ª sorteada em 25/6 e jogada em 26/6 (40 times, volta 4 dias depois), 2ª em 3/7 (32 times: 20 que passaram e 12 novos), 3ª em 17/7 (24: 16 e 8 novos), semifinal em 28/7 e final em 10/8 (6 times: os 3 vencedores vão à Copa da UEFA [inferência]); categoria 7 nas três primeiras e 8 depois; sem prêmios;
- `cqp13_intercontinental_tabela()` = 0x92b770 (world_club_cup.cpp, thiscall, ret 0x10): regra: a Copa Intercontinental (o campeão europeu contra o sul-americano [inferência]) é um jogo único em 1º de dezembro, sorteio em 10 de outubro, mando 4 (Tóquio [inferência]), categoria 6, desempate 3;
- `cqp13_interamericana_tabela()` = 0x632340 (inter_amer_cup.cpp, thiscall, ret 0x10): regra: a Copa Interamericana é em ida e volta a partir de 26 de janeiro do ano seguinte (7 dias), sorteio em 10 de outubro, categoria 8, desempate de ida e volta 3;
- `cqp13_concacaf_tabela()` = 0x4c0f00 (con_champ.cpp, thiscall, ret 0x10): regra: a Copa dos Campeões da CONCACAF tem 8 times em jogo único: quartas sorteadas em 1/8 e jogadas em 28/8, semifinal em 1/9 e final em 4/9; categoria 8, desempate 3; sem prêmio por rodada (−1);
- `cqq10_uefa_tabela()` = 0x9025c0 (uefa_cup.cpp, thiscall, ret 0x10): regra: a Copa da UEFA tem, só se há clubes da preliminar (+0xb2, os de coeficiente 10 ou mais [inferência]), uma preliminar sorteada em 28/6 e jogada em 6/7 (b times); depois: 1ª fase de qualificação sorteada em 28/7 e jogada em 10/8 (82 times, 0x52 − b/2 novos), 2ª em 14/9 (96 times, 55 novos), 1ª rodada em 19/10 (48 times), 2ª em 23/11 (32 times, 8 novos), 3ª em 1/3 do ano seguinte, oitavas em 15/3, quartas em 5/4 e a final em jogo único em 16/5 (mando 4, prêmio de 1,5 milhão ao campeão); ida e volta com 14 dias entre os jogos (7 na 3ª rodada e nas oitavas); categoria 7 na qualificação e 8 depois;
- `cqq10_intertoto_inscritos()` = 0x6336c0 (intertoto_cup.cpp, thiscall, ret 0): regra: entram na Intertoto os 60 clubes que as regras europeias puseram nela (copa europeia +0x1db = a Intertoto), pela ordem da tabela de clubes; cada um com a classe = o seu coeficiente (+0x1df), e a lista é ordenada pela classe (as sementes); com menos de 60, a caixa de erro;
- `cqq10_intertoto_lista()` = 0x633f70 (intertoto_cup.cpp, thiscall, ret 8): regra: a lista dos clubes da Intertoto (para quem chama): todos os inscritos, ou só os ainda vivos (situação 0 ou 1) quando pedido;
- `cqq11_europeia_clubes()` = 0x57a500 (eur_super_cup.cpp, thiscall, ret 0): regra: a Supercopa Europeia é entre o campeão da Liga dos Campeões e o da Recopa na edição de 1999 (a última Recopa) e, nas outras, o da Copa da UEFA; sem um dos dois, a caixa de erro;
- `cqq11_asiatica_clubes()` = 0x410a50 (asia_super_cup.cpp, thiscall, ret 0): regra: a Supercopa Asiática é entre o campeão da Liga dos Campeões asiática e o da Recopa asiática; se o mesmo clube ganhou as duas, entra o vice da Liga dos Campeões;
- `cqq11_c1_vagas()` = 0x63c9d0 (ita_c1_super.cpp, thiscall, ret 0): regra: a Supercopa da Série C1 começa com as duas vagas vazias; os clubes entram no fim da temporada [inferência: os campeões das duas chaves da C1];
- `cqq12_oceania_tabela()` = 0x797230 (oceania_club_champ.cpp, thiscall, ret 0x10): regra: o Campeonato de Clubes da Oceania tem dois grupos (fases −1 e 0, ids 0x41f e 0x420) de três rodadas em 10, 12 e 14 de janeiro do ano seguinte ao da copa, com os jogos fixos de cada grupo; o mata-mata (fase 1) em jogo único: semifinal em 17/1, terceiro lugar e final em 19/1 (mando 4 na final); categoria 8; desempate 0x283 na semifinal e 0x83 depois; sem prêmios;
- `cqq12_asia_clubes_times()` = 0x40b620 (asia_club_champ.cpp, thiscall, ret 0): regra: a Liga dos Campeões asiática tem 16 times, os classificados (+0xb2) na ordem em que foram escolhidos, todos com classe e situação 0;
- `cqq12_asia_recopa_times()` = 0x40d4d0 (asia_cup_winner.cpp, thiscall, ret 0): regra: a Recopa asiática tem 16 times, os classificados (+0xb2) na ordem em que foram escolhidos, todos com classe e situação 0;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (24):** 24 acusadas.
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 1dedd9dc, original 404adc50 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 280ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 281`

**Campos tocados:**
- liga +0x1c a liga de cima (índice; negativo no topo); +0x20 a liga de baixo (índice em [0xadadfc]; negativo na última); lê (cadeia_de_acessos)
- competição +4 o registro (+0 id); +0x40 o ano; clube +0x80 reputação; partida +0x1c/+0x20 os clubes, +0x3d/+0x3e [inferência: vencedor nos pênaltis], +0x43/+0x44 e +0x4b/+0x4c os placares; lê (notícias)
- copa +0x40 ano da copa; valor (palavra com sinal); lê (as quatro)
- rodada de copa (0x68 bytes) +0..+0x22 e +0x5c..+0x67; valor; grava (mata-mata e supercopas)
- rodada de grupo (0x41 bytes) +0 dia do ano, +2 ano relativo, +4 código, +5..+0xb jogo 0 vazio, +0x3d valor; valor; grava (grupos)
- copa +0x40 ano da copa; valor (palavra com sinal); lê (as três)
- rodada de copa (0x68 bytes) +0..+0x22 (com os confrontos fixos em +0x23 + 7 × vaga) e +0x5c..+0x67; valor; grava (mata-mata)
- rodada de grupo (0x41 bytes) +0 dia do ano, +2 ano relativo, +4 código, +5 + 7 × jogo os jogos, +0x3d valor; valor; grava (grupos asiáticos)
- torneio +0x40 ano do torneio; valor (palavra com sinal); lê (os três)
- rodada de copa (0x68 bytes) +0..+0x22 (com os confrontos fixos em +0x23 + 7 × vaga) e +0x5c..+0x67; valor; grava (mata-mata)
- rodada de grupo (0x41 bytes) +0 dia do ano, +2 ano relativo, +4 código, +5 + 7 × jogo os jogos, +0x3d valor; valor; grava (grupos)
- copa +0x40 ano da copa; valor (palavra com sinal); lê (as quatro)
- rodada de copa (0x68 bytes) +0..+0x22, +0x23..+0x29 (o dia 0 do confronto, CONCACAF) e +0x5c..+0x67; valor; grava
- copa +0x40 ano, +0xb2 clubes da preliminar (byte com sinal); valor; lê (tabela da UEFA)
- clube +0x1db copa europeia (id), +0x1df coeficiente europeu (byte) [inferência]; id/valor; lê (inscritos)
- copa +0x3e inscritos, +0xa7 lista (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (inscritos) e lê (lista)
- [0x9cf6f8] a Intertoto; id; lê
- copa +0x3e inscritos, +0xa7 lista (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (as três)
- copa +0x40 ano; valor; lê (Supercopa Europeia)
- [0x9cf6e8] Liga dos Campeões, [0x9cf6ec] Recopa, [0x9cf6f0] Copa da UEFA, [0x9cf64c]/[0x9cf650] as copas asiáticas; [0xae23d0] competições do banco; id; lê
- copa +0x40 ano; valor (palavra com sinal); lê (Oceania)
- rodada de grupo (0x41 bytes) e rodada de copa (0x68 bytes); valor; grava (Oceania)
- copa +0xb2 classificados (ponteiros de clube); ponteiro; lê (Ásia)
- copa +0x3e times, +0xa7 tabela (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (Ásia)

**Interessa a:** CMRTE
