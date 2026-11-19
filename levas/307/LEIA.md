# 307ª leva — a virada espanhola, o acesso e o rebaixamento; a virada francesa e a troca entre o CFA e o National; o acesso e o rebaixamento na Itália, a troca com a Série C2 e o reagrupamento da Série C1; a virada italiana; a troca com a Conference quando ela não é jogada; os clubes, o grupo A e a fase seguinte da Copa da Liga coreana

**Rotinas:**
- `spa_first_process_promotion_relegation_chain()` = 0x850700 (spa_first.cpp, thiscall, ret 4): regra: o acesso e o rebaixamento na Espanha: grava as posições finais da Primeira e da Segunda e troca os clubes marcados entre elas; com a Segunda B, grava as dela e dos três primeiros grupos e troca a Segunda com a Segunda B e com cada um desses grupos;
- `spa_first_exchange_second_division_clubs_unplayed()` = 0x8500c0 (spa_first.cpp, thiscall, ret 0): regra: sem a Segunda B, os rebaixados da Segunda descem para o grupo B1 e sobem tantos quantos desceram, primeiro da tabela da divisão inferior e depois dos clubes dos grupos B1 a B4; um time B nunca sobe para a divisão do seu clube principal, e o time B cujo clube principal está agora na Segunda é rebaixado à força, com notícia;
- `spa_first_start_new_season()` = 0x84feb0 (spa_first.cpp, thiscall, ret 0): regra: a virada espanhola, puxada pela Primeira: os conflitos dos times B (com a Segunda B), o acesso e o rebaixamento e a troca com a Segunda B (ou, sem ela, com a divisão inferior); a Primeira refeita (ano seguinte, regras, 20 clubes, jogos, ordem) e as viradas da Segunda e da Segunda B (ou da divisão inferior); depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros;
- `fra_first_exchange_cfa_national_clubs()` = 0x5bc780 (fra_first.cpp, thiscall, ret 0): regra: os melhores clubes do CFA (pela reputação, embaralhados de 4 em 4) sobem para o National, seis na virada de 2001 e quatro depois; os rebaixados do National voltam para o CFA;
- `fra_first_start_new_season()` = 0x5bc4d0 (fra_first.cpp, thiscall, ret 0): regra: a virada francesa, puxada pela Primeira: acesso e rebaixamento, a troca entre o CFA e o National, a Primeira refeita (ano seguinte, regras, 18 clubes, jogos, ordem), o teto de divisão dos clubes do National, as viradas da Segunda e do National; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros; por fim um CFA e uma liga inferior do ano novo são montados e descartados em seguida;
- `ita_ser_a_process_promotion_relegation_chain()` = 0x642410 (ita_ser_a.cpp, thiscall, ret 4): regra: o acesso e o rebaixamento na Itália: grava as posições finais da Série A, B, C1 A e C1 B; quatro sobem e quatro descem entre A e B; trocas entre a B e cada grupo da C1; com a C2 carregada, as posições dos seus três grupos e as trocas entre cada grupo da C1 e cada grupo da C2;
- `ita_ser_a_exchange_serie_c2_clubs_unplayed()` = 0x641fc0 (ita_ser_a.cpp, thiscall, ret 0): regra: sem a Série C2 carregada, os seis melhores clubes da C2 (pela reputação, embaralhados de seis em seis) sobem para a C1 A e os rebaixados dos dois grupos da C1 descem para a C2 A;
- `ita_ser_a_regroup_serie_c1_clubs()` = 0x642630 (ita_ser_a.cpp, thiscall, ret 0): regra: os 36 clubes da Série C1 são reagrupados de norte a sul (pela cidade do estádio): os 18 primeiros na C1 B e os outros na C1 A; o técnico humano de um clube que muda de grupo recebe a notícia;
- `ita_ser_a_start_new_season()` = 0x641700 (ita_ser_a.cpp, thiscall, ret 0): regra: a virada italiana, puxada pela Série A: acesso e rebaixamento, a troca com o nível de baixo (a Série D e o reagrupamento da C2 com a C2 carregada; senão a troca com a C2), o reagrupamento da C1, a Série A refeita (ano seguinte, regras, 18 clubes, jogos, ordem), o teto de divisão nos grupos mais baixos carregados e as viradas da B, da C1 A, da C1 B (e da C2); depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros;
- `eng_prm_exchange_conference_clubs_unplayed()` = 0x574e60 (eng_prm.cpp, thiscall, ret 0): regra: sem a Conference jogada, o clube mais forte dela (pela reputação) sobe para a Third Division se o estádio cumpre as exigências da Third, e os rebaixados da Third descem para a Conference; senão o último da Third fica e o clube da Conference recebe a notícia;
- `kor_league_cup_select_teams()` = 0x66cf20 (kor_league_cup.cpp, thiscall, ret 0): regra: a Copa da Liga coreana tem os dez clubes da K League, ordenados pela posição da última temporada;
- `kor_league_cup_create_group_a_table()` = 0x66c770 (kor_league_cup.cpp, thiscall, ret 0): regra: o grupo A da Copa da Liga coreana (a própria copa) tem as cinco cabeças pares (1a, 3a, 5a, 7a e 9a da ordem);
- `kor_league_cup_advance_stage()` = 0x66cad0 (kor_league_cup.cpp, thiscall, ret 0): regra: depois dos grupos a Copa da Liga coreana passa ao mata-mata (montado ao sair da fase 0); passar da última fase é erro;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (10):** 10 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (570739b5).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 31731bcf, original 8e889891 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 306ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 307`

**Campos tocados:**
- linha da tabela +0x37 (status 3 = rebaixado; a troca marca o time B rebaixado à força); clube +0x57 e +0x5b (movidos); valor; grava
- clube +0x57, +0x5b, +0x60 (divisão de reservas: o CFA); linha da tabela +0x37 (status 3 = rebaixado); valor/ponteiro; lê/grava
- clube +0x57 (divisão: o grupo da C1), +0x5b; linha da tabela +0x37 (status 3 = rebaixado); valor/ponteiro; lê/grava
- liga +0x4c (grava 1 no fim da virada), +0xc e +0x2c (zerados), +0x30, +0x40, +0xb1, +0xba; valor/ponteiro; grava
- liga +0xe2/+0xe4 (exigências do estádio); linha da tabela +0x37 (3 = rebaixado, 0xfe = fica); clube +0x57, +0x5b; valor; lê/grava
- copa coreana +0xee (lista dos dez clubes), +0xf2 (lugares); liga +0x3e, +0xb1, +0x30; valor/ponteiro; grava
