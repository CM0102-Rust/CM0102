# 275ª leva — a MLS: o destino de cada clube no fim das fases, o resultado nas tabelas das duas conferências e a conferência do Jogo das Estrelas; as regras das quatro divisões inglesas: turnos, pontos, acesso, mata-mata, rebaixamento e desempates; a tabela das quatro divisões inglesas: os clubes, o fundo de prêmios e o dinheiro da TV; as regras das divisões italianas: turnos, pontos, acesso, play-off, play-out, rebaixamento e o jogo-desempate; a tabela das divisões italianas: os clubes, o fundo de prêmios e a TV da Série A; as regras das três divisões francesas: turnos, pontos, acesso e rebaixamento pelo ano (a primeira passa a 20 clubes em 2002) e desempates; a tabela das três divisões francesas: os clubes, o fundo de prêmios e a TV pela divisão de origem; as regras das divisões espanholas: turnos, pontos, acesso, rebaixamento e o confronto direto primeiro nos desempates; a Primera espanhola: a tabela, a TV e a situação nas estatísticas; a Série C2/A: os registros pela situação; as tabelas da CFA e da divisão de baixo francesa; a situação nas estatísticas da Série C1 italiana

**Rotinas:**
- `cqk9_destino_na_mls()` = 0x908200 (usa_mls.cpp, thiscall): MLS: na temporada e nas duas primeiras rodadas dos play-offs, quem avança ganha o registro de classificação (fase 0xa0, rodada 0x78, peso 0x28) e quem fica de fora o de não classificado; na final, o vencedor ganha o registro da final (peso 0x50) e os outros o da rodada, com peso de 50 a 70; depois, o campeão ganha o de campeão;
- `cqk9_resultado_na_mls()` = 0x9099d0 (usa_mls.cpp, thiscall): MLS: cada jogo vai para a tabela geral e, na temporada regular, também para as tabelas das duas conferências (Leste e Oeste [inferência]), sem contar duas vezes as estatísticas;
- `cqk9_conferencia_das_estrelas()` = 0x90a000 (usa_mls.cpp, cdecl): Jogo das Estrelas da MLS: Leste (0x413) para New England, MetroStars, Tampa Bay, D.C. United, Columbus e Miami; Oeste (0x414) para Colorado, Dallas, Kansas City, Los Angeles, San Jose e Chicago; os de fora da MLS, nenhuma;
- `regras_da_premier()` = 0x574b70 (eng_prm.cpp, thiscall, ret 0): a Premier (20 clubes): 2 turnos, 3 pontos pela vitória e 1 pelo empate; ninguém sobe, os 3 últimos caem direto para a First; desempates saldo, gols pró e posição [inferência pelos códigos 1, 2, 3]; monta a tabela de rodadas;
- `regras_da_first()` = 0x572d90 (eng_first.cpp, thiscall, ret 0): a First (24): os 2 primeiros sobem direto, do 3º ao 6º vão ao mata-mata de acesso, os 3 últimos caem para a Second; 5 reservas e 3 substituições; 2 turnos, 3 pontos;
- `regras_da_second()` = 0x576780 (eng_second.cpp, thiscall, ret 0): a Second (24): 2 sobem direto, do 3º ao 6º ao mata-mata, os 4 últimos caem para a Third;
- `regras_da_third()` = 0x5780c0 (eng_third.cpp, thiscall, ret 0): a Third (24): 3 sobem direto, do 4º ao 7º ao mata-mata, o último cai para a Conference quando a nação tem as divisões de baixo carregadas [inferência pelo bit 4 da nação] (senão não há liga abaixo);
- `tabela_da_premier()` = 0x574a20 (eng_prm.cpp, thiscall, ret 0): a Premier tem 20 clubes; o fundo de prêmios parte de 250.000 por posição; a TV paga 12.000.000 a cada clube (5.000.000 até 1999);
- `tabela_da_second()` = 0x576630 (eng_second.cpp, thiscall, ret 0): a Second tem 24 clubes; fundo de 20.000 por posição; a TV paga 2.500.000 a cada clube (300.000 até 1999);
- `tabela_da_third()` = 0x577f70 (eng_third.cpp, thiscall, ret 0): a Third tem 24 clubes; fundo de 10.000 por posição; a TV paga 1.000.000 a cada clube (200.000 até 1999);
- `tabela_da_first()` = 0x572bf0 (eng_first.cpp, thiscall, ret 0): a First tem 24 clubes; fundo de 70.000 por posição; a TV paga pela divisão de onde o clube veio: 5.877.777 aos que caíram da Premier e 4.099.999 aos que já estavam na First e aos que subiram da Second (1.000.000 a todos até 1999);
- `regras_da_serie_a()` = 0x641640 (ita_ser_a.cpp, thiscall, ret 0): a Série A (18 clubes): 2 turnos, 3 pontos pela vitória e 1 pelo empate, ninguém sobe, os 4 últimos caem para a B; 7 reservas e 3 substituições; desempates 1, 2 e 4 [códigos; inferência: saldo, gols pró, confronto direto]; o empate em pontos na fronteira de uma vaga vai ao jogo-desempate (+0xe6); ordem fixa das rodadas;
- `regras_da_serie_b()` = 0x6468a0 (ita_ser_b.cpp, thiscall, ret 0): a Série B (20): os 4 primeiros sobem para a A, os 4 últimos caem para a C1; o resto como a A (jogo-desempate também);
- `regras_da_c1a()` = 0x64b770 (ita_ser_c1a.cpp, thiscall, ret 0): a Série C1/A (18): o campeão sobe para a B, do 2º ao 5º vão ao play-off de acesso, do 14º ao 17º ao play-out de permanência e o último cai; desempates 1 e 2; a liga de baixo gravada é a C1/B (assim no original);
- `regras_da_c1b()` = 0x64f920 (ita_ser_c1b.cpp, thiscall, ret 0): a Série C1/B (18): como a C1/A; abaixo a C2/A quando a nação tem as divisões de baixo carregadas [inferência pelo bit 4 da nação];
- `tabela_da_serie_a()` = 0x641510 (ita_ser_a.cpp, thiscall, ret 0): a Série A tem 18 clubes; o fundo de prêmios parte de 250.000 por posição (pago pela tabela no fim da liga); a TV paga 5.000.000 a cada clube;
- `tabela_da_serie_b()` = 0x646780 (ita_ser_b.cpp, thiscall, ret 0): a Série B tem 20 clubes; fundo de 70.000 por posição; sem TV;
- `tabela_da_c1a()` = 0x64b670 (ita_ser_c1a.cpp, thiscall, ret 0): a Série C1/A tem 18 clubes; sem fundo de prêmios nem TV;
- `tabela_da_c1b()` = 0x64f820 (ita_ser_c1b.cpp, thiscall, ret 0): a Série C1/B tem 18 clubes; sem fundo de prêmios nem TV;
- `regras_da_franca_1()` = 0x5bc410 (fra_first.cpp, thiscall, ret 0): a primeira divisão: 2 turnos, 3 pontos pela vitória e 1 pelo empate; ninguém sobe; caem 2 na temporada 2001 (18 clubes, a liga cresce para 20) e 3 depois; desempates 1, 2 e 3 [códigos]; 5 reservas e 3 substituições; ordem das rodadas dividida ao meio (+0xd9 = 0x83);
- `regras_da_franca_2()` = 0x5bf0a0 (fra_second.cpp, thiscall, ret 0): a segunda: sobem 4 e caem 2 em 2001, depois sobem 3 e caem 3; o resto como a primeira;
- `regras_da_franca_3()` = 0x5c0700 (fra_third.cpp, thiscall, ret 0): o National: sobem 4 em 2001 e 3 depois, caem 4 (sem liga abaixo no jogo); 3 reservas e 3 substituições;
- `tabela_da_franca_1()` = 0x5bc2b0 (fra_first.cpp, thiscall, ret 0): a primeira tem 18 clubes na temporada 2001 e 20 depois; fundo de 200.000 por posição; a TV paga 6.000.000 a cada clube;
- `tabela_da_franca_2()` = 0x5bef10 (fra_second.cpp, thiscall, ret 0): a segunda tem 20 clubes; sem fundo de prêmios; a TV paga pela divisão de onde o clube veio: 2.000.000 aos que caíram da primeira e 1.500.000 aos que ficaram e aos que subiram do National;
- `tabela_da_franca_3()` = 0x5c05d0 (fra_third.cpp, thiscall, ret 0): o National tem 20 clubes; fundo de 20.000 por posição; a TV paga 200.000 a cada clube;
- `regras_da_primera()` = 0x84fdf0 (spa_first.cpp, thiscall, ret 0): a Primera (20 clubes, 38 rodadas): 2 turnos, 3 pontos pela vitória e 1 pelo empate; ninguém sobe, os 3 últimos caem para a Segunda; desempates 4, 1, 2 e 3 [códigos; inferência: confronto direto, saldo, gols pró, vitórias]; 7 reservas e 3 substituições; ordem fixa das rodadas;
- `regras_da_segunda()` = 0x855cc0 (spa_second.cpp, thiscall, ret 0): a Segunda (22): os 3 primeiros sobem direto, os 4 últimos caem para a Segunda B quando ela está carregada [inferência pelo bit 4 da nação]; os times B não sobem (marcados pela tabela, 0x855b40);
- `regras_da_divisao_de_baixo()` = 0x853790 (spa_lower.cpp, thiscall, ret 0): a divisão de baixo (22: times B e os melhores clubes menores pela reputação): 2 sobem direto e 2 vão ao play-off de acesso, 4 caem; a liga de cima gravada é a Primera (assim no original);
- `tabela_da_primera()` = 0x84fc70 (spa_first.cpp, thiscall, ret 0): a Primera tem 20 clubes; antes, os clubes da divisão de baixo espanhola passam à divisão de baixo genérica; fundo de prêmios de 250.000 por posição; a TV paga 5.000.000 a cada clube;
- `registros_pela_situacao_na_c2a()` = 0x654180 (ita_ser_c2a.cpp, thiscall, ret 0x18): os registros da Série C2/A: na tabela, o título, o acesso (peso 100), a vaga no play-off (peso 40) e o rebaixamento; no play-off de acesso, quem vence sobe (peso 50) e quem cai fica com o registro da eliminação (peso pela rodada); no play-out, quem vence se salva e quem perde cai;
- `situacao_na_primera()` = 0x850880 (spa_first.cpp, thiscall, ret 0x14): a posição atual, a melhor e a pior possíveis do clube nas estatísticas da Primera; as posições 17 e 18 viram 16 na melhor e 19 na pior [inferência: não formam faixa própria];
- `tabela_do_grupo_da_cfa()` = 0x5b9fa0 (fra_cfa.cpp, thiscall, ret 4): um grupo da CFA tem 18 clubes (4 grupos, sorteados pelo construtor); os clubes que estão na primeira ou na segunda divisão levam a marca da linha [inferência: os times reserva dos profissionais, que não sobem];
- `tabela_da_divisao_de_baixo_francesa()` = 0x5be230 (fra_lower.cpp, thiscall, ret 4): a divisão de baixo francesa tem 20 clubes (escolhidos pelo construtor); os que estão no National levam a marca;
- `situacao_na_c1a()` = 0x64bd80 (ita_ser_c1a.cpp, thiscall, ret 0x14): a previsão de posição do clube na C1/A: na liga, a melhor e a pior posição viram faixas (2 a 5 = o play-off de acesso: a melhor 2, a pior 5; 14 a 17 = o play-out: a melhor 14, a pior 17); no play-off, as posições + 1; no play-out, quem vence fica em 14 e quem perde em 16;
- `situacao_na_c1b()` = 0x64ff40 (ita_ser_c1b.cpp, thiscall, ret 0x14): a mesma regra na C1/B;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (30):** 30 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (bdb97d45).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 1dedd9dc, original c62bf4a7 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 274ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 275`

**Campos tocados:**
- liga +4: registro da competição no banco; ponteiro; lê (vai nos registros de carreira)
- liga +8: bloco de estatísticas (Comp_Stats); ponteiro; lê, grava 0 e devolve (add_result)
- liga +0xc: vetor de estágios (as duas conferências); ponteiro; lê
- liga +0x30: estágio atual (1 = a temporada regular da MLS [inferência]); valor; lê
- jogo +0x42: marca da conferência (-1 geral, 0 e 1 as conferências) [inferência]; valor; lê e grava
- jogo +0x30 / +0x32 / +0x34: rodada, fase e a rodada da escala; valor (palavra); lê
- clube +0: id; clube +0x57: a liga do clube (club_comp); ponteiro; lê
- [0x9cf590] chave da MLS; [0x9d037c..0x9d03a8] chaves dos doze clubes; id; lê
- [0xdd7f00]: Staff_Records; ponteiro; lê
- liga +0x3c turnos; +0xc2/+0xc3 pontos por vitória/empate; +0xc4 categoria das partidas (2 = liga); +0xc5..+0xc7 desempates; valor; grava (as quatro)
- liga +0xbe/+0xbf/+0xc0/+0xc1 vagas de acesso direto, mata-mata de acesso, play-off de permanência e rebaixamento direto; valor; grava (as quatro)
- liga +0x1c/+0x20 a liga de cima/de baixo; id (−1 nenhuma); grava (as quatro)
- liga +0x42 tipo (1 clubes nacionais), +0x49 reservas, +0x4a substituições, +0x52 [sem nome], +0xab/+0xdb palavras copiadas nas partidas; valor; grava
- liga +0xba tabela de rodadas, +0xa9 rodadas, +0x3a código da fase; ponteiro e valor; grava pela virtual +0x3c (as quatro)
- nação +0x11c bit 4 as divisões de baixo carregadas [inferência]; valor; lê (regras_da_third)
- liga +0x3e clubes; valor; grava (as quatro)
- liga +0xb1 tabela (linhas de 0x3b bytes, +0 clube); ponteiro; grava (as quatro, pela 0x687430)
- liga +0x28 fundo de prêmios; valor; grava; +0xe1 = 1 [sem nome]; valor; grava (as quatro)
- clube +0x57 competição (id no +0); ponteiro; lê (as quatro); +0x5b competição anterior; lê pela 0x683010 (tabela_da_first)
- [0xae2c92] ano do jogo; valor; lê (as quatro)
- liga +0x3c turnos; +0xc2/+0xc3 pontos; +0xc4 categoria das partidas; +0xc5..+0xc8 desempates; valor; grava (as quatro)
- liga +0xbe/+0xbf/+0xc0/+0xc1 acesso direto, play-off de acesso, play-out, rebaixamento; valor; grava
- liga +0x1c/+0x20 a liga de cima/de baixo; id; grava
- liga +0xd9 bits de regra (0x40 = ordem fixa das rodadas); valor; lê e grava (as quatro)
- liga +0xe6 o empate na fronteira vai ao jogo-desempate; valor; grava (regras_da_serie_a, regras_da_serie_b)
- liga +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; valor; grava; +0xba, +0xa9, +0x3a pela virtual +0x3c
- nação +0x11c bit 4 [inferência: divisões de baixo carregadas]; valor; lê (regras_da_c1b)
- liga +0x3e clubes; +0xb1 tabela (linhas de 0x3b bytes, +0 clube); valor e ponteiro; grava (as quatro)
- liga +0x28 fundo de prêmios; +0xe1 modo do prêmio (1 = pela tabela, 2 = nenhum, 3 = pela competição, pela 0x682e10); valor; grava (tabela_da_serie_a, tabela_da_serie_b)
- clube +0x57 competição (id no +0); ponteiro; lê (as quatro)
- liga +0x40 ano da liga; valor; lê (as três)
- liga +0x3c turnos; +0xc2/+0xc3 pontos; +0xc4 categoria; +0xc5..+0xc7 desempates; +0xbe..+0xc1 vagas; valor; grava (as três)
- liga +0xd9 bits de regra = 0x83; valor; grava (as três)
- liga +0x1c/+0x20 a liga de cima/de baixo; id; grava
- liga +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; valor; grava; +0xa7 = −1 (regras_da_franca_3); +0xba, +0xa9, +0x3a pela virtual +0x3c
- liga +0x3e clubes; +0xb1 tabela (linhas de 0x3b bytes, +0 clube); valor e ponteiro; grava (as três)
- liga +0x28 fundo de prêmios; +0xe1 modo do prêmio (1 = pela tabela, 2 = nenhum); valor; grava (as três)
- liga +0x40 ano da liga; valor; lê; +0xa7 = −1; valor; grava (tabela_da_franca_1)
- clube +0x57 competição (id no +0); ponteiro; lê; +0x5b competição anterior; lê pela 0x683010
- liga +0x3c turnos; +0xc2/+0xc3 pontos; +0xc4 categoria; +0xc5..+0xc8 desempates; +0xbe..+0xc1 vagas; valor; grava (as três)
- liga +0xd9 bits de regra = 0x43; valor; grava (as três)
- liga +0x1c/+0x20 a liga de cima/de baixo; id; grava
- liga +0xa9 rodadas (38 na Primera, antes da virtual); valor; grava (regras_da_primera)
- liga +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; valor; grava; +0xba, +0x3a pela virtual +0x3c
- nação +0x11c bit 4 [inferência: divisões de baixo carregadas]; valor; lê (regras_da_segunda)
- clube +0x57 competição (id no +0); ponteiro; lê e grava (tabela_da_primera: a divisão de baixo espanhola vira a genérica, [0xae23d0] + [0x9cf764] × 0x6b)
- liga +0x3e, +0xb1, +0x28, +0xe1; valor e ponteiro; grava (tabela_da_primera)
- linha da tabela +0x37 situação (0 subiu, 3 caiu, 0xfe fora); valor; grava (registros_pela_situacao_na_c2a)
- jogo +0x30 rodada, +0x32 fase, +0x34 rodada para o peso; valor; lê (registros_pela_situacao_na_c2a)
- Comp_Stats entrada do clube +0x73 posição atual, +0x74 melhor, +0x75 pior; valor; grava (situacao_na_primera)
- linha da tabela +0 clube, +0x38 marca (1 = clube de divisão maior); ponteiro e valor; grava pela 0x687430 (as duas tabelas)
- clube +0x57 competição (id no +0); ponteiro; lê (as duas tabelas)
- Comp_Stats entrada do clube +0x73 posição atual, +0x74 melhor, +0x75 pior; valor; grava (situacao_na_c1a, situacao_na_c1b)

**Interessa a:** CMRTE
