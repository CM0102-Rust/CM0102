# 257ª leva — as linhas da nação-chave: a linha de uma nação, a linha do clube e o ano da temporada do clube; a temporada do clube e a nação principal: se o clube está na temporada e as datas dela, o continente principal e a nação principal; as estatísticas do jogador: a estatística de uma categoria e as notas combinadas, a ordem dos idiomas e a ordem da história por clube; as cláusulas do contrato: a cláusula de rescisão ativa, o período protegido da FIFA e o tipo de bônus permitido; o adiamento pelo clima: a decisão de adiar ou abandonar o jogo, as finanças do clube e a ordem das reservas do estádio; o clima da partida: o vento, a chuva e a temperatura do jogo, a temperatura pela faixa e o estado do gramado; a riqueza do clube e duas ordens: a faixa das finanças, a ordem dos jogadores pela habilidade e a dos estádios pela capacidade; os programas de treino padrão: os sete do computador e os dez de cada técnico humano no começo do jogo, o tipo do programa e o programa do humano por vaga; as fichas de treino iniciais dos jogadores, o banco de jogadores livres do regen e se há reserva de não jogador; as suspensões dos órgãos disciplinares: a suspensão do cartão vermelho, a virada da temporada do órgão e a compactação das suas entradas; as casas da formação: gravar a coordenada da casa e escolher as posições, a posição repetida e o tipo de marcação; a ordem dos clubes da liga para a tabela de jogos e os clubes possíveis de cada lado de um jogo da copa; a nota-alvo da busca de jogadores do clube e o histórico de jogos do jogador acompanhado: o jogo da vez e a soma dos gols

**Rotinas:**
- `linha_da_nacao()` = 0x669340 (key_nation.cpp, cdecl): a primeira linha da tabela das 35 nações-chave com a nação dada, ou 4;
- `linha_da_nacao_do_clube()` = 0x669360 (key_nation.cpp, cdecl): a linha da nação escolhida do clube; seleção, 1; nação não escolhida, a linha do continente dela, ou 3;
- `ano_da_temporada_do_clube()` = 0x669950 (key_nation.cpp, cdecl): o ano da temporada do clube: o ano anterior antes do dia de início da temporada da linha dele;
- `clube_na_temporada()` = 0x669bd0 (key_nation.cpp, cdecl): pelos dias de início e fim da linha da nação do clube e o dia de hoje: se está na temporada e as datas de início e fim com o ano certo;
- `continente_principal()` = 0x6695c0 (key_nation.cpp, cdecl): o continente da nação do humano, ou o da primeira nação principal da tabela;
- `nacao_principal()` = 0x669600 (key_nation.cpp, cdecl): a nação do humano, ou a primeira nação principal da tabela;
- `estatistica_da_categoria()` = 0x7c1e30 (player_stats.cpp, thiscall): a estatística pedida do registro da categoria (sem registro, o padrão para a nota e a média), ou as notas combinadas das categorias 2 a 5;
- `ordem_dos_idiomas()` = 0x7c6d40 (player_stats.cpp, cdecl): a ordem dos registros de idioma: pelo id da pessoa e pelo idioma, com 0xff valendo qualquer idioma;
- `ordem_da_historia_por_clube()` = 0x7c6d90 (player_stats.cpp, cdecl): a ordem da história do jogador pelo dword +0xa e depois pelo +4;
- `clausula_de_rescisao_ativa()` = 0x867b50 (staff_contracts.cpp, thiscall): 1 com uma das três cláusulas de rescisão ativa no contrato;
- `periodo_protegido()` = 0x868210 (staff_contracts.cpp, thiscall): se o contrato ainda está no período protegido das regras novas da FIFA: 3 anos abaixo de 28 anos de idade na assinatura, senão 2;
- `tipo_de_bonus_permitido()` = 0x867aa0 (staff_contracts.cpp, cdecl): se o tipo de bônus vale para o jogador: o tipo 2 só para goleiros, os outros só para quem não é goleiro;
- `adiar_pelo_clima()` = 0x923630 (weather.cpp, thiscall, ret 4): só jogos da liga do mandante, com estádio descoberto (+0x4c) com cidade e lugares e fora do fim da temporada da nação-chave; na fatia da noite, sorteio(1000/2500/5000 pela faixa das finanças) zero devolve 0x12; pelo clima (0x923010): chuva 5 com 1 a 13 graus em estádio de até 20000 lugares sorteia o adiamento (9); chuva 8 e 9 (gelo), cadeias de sorteio que devolvem 0, 5 ou 6;
- `financas_do_clube()` = 0x5a0590 (finance.cpp, thiscall, ret 4): o registro de finanças (0x167 bytes) pelo id do clube; clube B com parceiro A, o do A (0x540a50); clube nulo, caixa de erro;
- `ordem_das_reservas_do_estadio()` = 0x85e380 (stadium.cpp, cdecl): qsort das reservas: pela data (+4), depois pelo id do estádio (+0);
- `clima_da_partida()` = 0x923010 (weather.cpp, thiscall, ret 8): a cidade do estádio (senão a do mandante, senão a capital); o vento da cidade, a região e o período pelo dia; vento (0..4), chuva (0, 1, 2, 3, 5) e faixa da temperatura pelos limites do período; abaixo de zero, a chuva vira neve/gelo (6..9) pelo vento da cidade;
- `temperatura_do_jogo()` = 0x9232c0 (weather.cpp, thiscall, ret 8): (dia × faixa) módulo 6/7/8 mais a base da faixa (−5 a 35; outra faixa −100); na fatia da noite, − 2 abaixo de 10, senão × 0,8;
- `gramado_do_jogo()` = 0x923970 (weather.cpp, thiscall, ret 8): pela chuva, a temperatura, a marca e os lugares do estádio e um resto do jogo módulo 20: 0, 1, 2, 4, 8, 0x10, 0x20 ou 0x40; estádio coberto, 0;
- `faixa_de_riqueza()` = 0x5986b0 (finance.cpp, thiscall): seleção, 0; pelo saldo de 64 bits contra a reputação R do clube: acima de 2500·R + 5.000.000, 2; acima de 1000·R + 3.500.000, 1; abaixo de −2.500.000 − 1500·R, −2; abaixo de −250·R (R < 5000) ou (3750 − R)·1000, −1; senão 0;
- `ordem_por_habilidade()` = 0x5d6220 (game.cpp, cdecl): qsort dos dados de jogador: habilidade atual (+5), depois potencial (+7), crescente;
- `ordem_pela_capacidade()` = 0x85e350 (stadium.cpp, cdecl): qsort dos locais: lugares (+0x3c) do estádio (+0xc), do maior para o menor; sem estádio, 0;
- `programas_padrao()` = 0x8b3d40 (training_manager.cpp, thiscall): monta os sete programas do computador (o tipo k na vaga k) e os dez programas de cada um dos 16 técnicos humanos, e dá os nomes padrão;
- `tipo_do_programa()` = 0x8b4e50 (training_manager.cpp, thiscall, ret 8): grava a vaga e o tipo do programa e os cinco níveis das categorias pela mistura do tipo (geral, uma categoria forte, goleiros ou sem treino);
- `programa_do_humano()` = 0x8b3930 (training_manager.cpp, thiscall, ret 8): o programa da vaga 0 a 9 (fora disso, 1) de um técnico humano; fora dos 16 humanos, a caixa de erro e 0;
- `fichas_de_treino_iniciais()` = 0x8b3c50 (training_manager.cpp, thiscall): aloca a ficha de treino de 6 bytes de cada ficha de jogador e põe os goleiros (nota de goleiro de 15 para cima) no programa 6 e os outros no 1;
- `guardar_jogador_no_banco()` = 0x7a8350 (player_regen.cpp, thiscall, ret 0xc): acrescenta ao banco de jogadores livres o id da ficha de jogador de uma pessoa que deixa de jogar, com a nação e a segunda nação, crescendo a tabela de 250 em 250;
- `tem_reserva_de_tecnico()` = 0x7a6e20 (player_regen.cpp, thiscall): se a reserva de fichas de não jogador tem alguma;
- `suspensao_do_vermelho()` = 0x7e8e10 (ruling_body.cpp, thiscall, ret 0xc): monta a suspensão do cartão vermelho: os jogos ou os dias pelo código da falta (9 a 12) e pela escala do órgão, a data de hoje e o jogador; 0 sem suspensão;
- `nova_temporada_do_orgao()` = 0x7e7ed0 (ruling_body.cpp, thiscall): na virada da temporada, solta as suspensões de 2 (ou 4) anos, libera as entradas sem suspensão, zera os cartões das outras e compacta a tabela;
- `compactar_disciplina_do_orgao()` = 0x7e9050 (ruling_body.cpp, thiscall): tira as entradas livres da tabela disciplinar do órgão, acerta o índice guardado de cada jogador que muda de lugar e realoca com folga de 100;
- `coordenada_da_casa()` = 0x5b3560 (formation.cpp, thiscall, ret 0x10): grava a coordenada da casa na fase e na faixa da bola e escolhe a posição geral e a extra pelas posições mais perto das 12 coordenadas da casa;
- `posicao_repetida_na_formacao()` = 0x5b3460 (formation.cpp, thiscall, ret 8): acha outra casa com a mesma posição geral e marca a saída quando a marca dela é menor;
- `tipo_de_marcacao()` = 0x5b57d0 (formation.cpp, thiscall, ret 8): troca um tipo de marcação da casa (bits 0..2 ou 3..4 do byte +0x5b4) e zera +0x187f;
- `sortear_ordem_dos_clubes()` = 0x686a70 (league.cpp, thiscall, ret 4): embaralha a tabela da liga pelo rand semeado e põe os clubes do mesmo estádio, dos estádios ligados e o Celtic e o Rangers em vagas pareadas;
- `clubes_do_lado_do_jogo()` = 0x521790 (Cup.cpp, thiscall, ret 0x10): o clube de um lado do jogo ou, vazio, os clubes que ainda podem chegar a ele (conta no global 0xae1880);
- `clubes_possiveis_do_jogo()` = 0x5219b0 (Cup.cpp, thiscall, ret 0x10): acrescenta à saída os clubes que podem jogar o jogo, descendo pelos jogos de onde vêm os lados vazios;
- `nota_alvo_do_clube()` = 0x7bca90 (player_search.cpp, cdecl): a nota que o clube procura: a base pela reputação, o dinheiro, a nação e a liga, por faixas, e os ajustes pelo elenco e pela riqueza; no mínimo 500;
- `jogo_do_historico()` = 0x7bd830 (player_search.cpp, thiscall, ret 4): o jogo i (0x29 bytes) do anel de 10 do jogador acompanhado, ou 0 além da conta;
- `gols_do_historico()` = 0x7bd870 (player_search.cpp, thiscall, ret 0): a soma dos gols dos jogos guardados no anel;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (35):** 32 acusadas. `cpu4_periodo_de_dois_anos` escapou: o período protegido pela idade só vale com as regras novas da FIFA ([0xae233c]), que não ligam antes de 2002-06-01 na carreira [inferência]. `cpu9_goleiro_so_com_20` escapou: o ramo do goleiro roda 512 vezes, mas a vaga da ficha inicial só pesa no clube humano; no computador a primeira atualização do treino (0x8b1750) refaz a vaga com a mesma regra [inferência]. `cpu11_sem_dobro` escapou: a distância dobrada roda 4096 vezes, e o comparador lado a lado também não viu diferença: nas formações carregadas a mutação é equivalente (as casas escolhidas não mudam) [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (bc5827d1).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 345e9799, original 24a0d51a (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 256ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 257`

**Campos tocados:**
- tabela das nações-chave 0xb63d60 (35 × 72 bytes): nação +0, continente +4, dia de início +0x15, dword +0x17; lê (as três)
- clube +0x53: nação; ponteiro; lê (linha_da_nacao_do_clube)
- nação +0x11c: nação escolhida para jogar; valor; +0x71: continente; ponteiro; lê (linha_da_nacao_do_clube)
- [0xae2c90] dia e [0xae2c92] ano de hoje; valor; lê (ano_da_temporada_do_clube)
- tabela das nações-chave 0xb63d60 (35 × 72 bytes): nação +0, data +0x2a (início), data +0x39 (fim); lê (clube_na_temporada)
- clube +0x53: nação; ponteiro; lê (clube_na_temporada)
- nação +0x11c bit 2: nação principal; +0x71: continente; lê (continente_principal, nacao_principal)
- [0xae2d08] nação do humano; ponteiro; [0xae2c90]/[0xae2c92] dia e ano de hoje; lê
- pessoa +0: id; valor; lê (estatistica_da_categoria); [0xae236c] número de pessoas com estatística; lê
- registro da categoria (0x20 bytes) +0: jogos; +0xe: total (nota); lê (estatistica_da_categoria)
- registro de idioma (6 bytes) +0: id da pessoa; +4: idioma; lê (ordem_dos_idiomas)
- registro da história +4 e +0xa; valor; lê (ordem_da_historia_por_clube)
- contrato +0x1c, +0x1d, +0x1f: cláusulas de rescisão; valor; lê (clausula_de_rescisao_ativa)
- contrato +0: id da pessoa; id; +0x25: data do contrato; valor; lê (periodo_protegido)
- [0xae233c] regras novas da FIFA ligadas; [0xae2330] data das regras; valor; lê (periodo_protegido)
- pessoa +0x61: ficha de jogador; ponteiro; ficha +0xf: posição (0x14 goleiro); lê (tipo_de_bonus_permitido)
- jogo +0x14 competição, +0x18 estádio, +0x1c mandante, +0x2a dia, +0x3f fatia do dia; ponteiros/valores; lê (adiar_pelo_clima)
- estádio +0x38 cidade, +0x3c lugares, +0x4c coberto, +0x4d marca; ponteiro/valores; lê (adiar_pelo_clima)
- clube +0 id, +0x57 liga, +0x64 marca do mandante; id/ponteiro/valor; lê (adiar_pelo_clima, financas_do_clube)
- [0xb63d60] nações-chave (0x48 bytes) +0x39 dia do fim, +0x3b ano; valores; lê (adiar_pelo_clima)
- finanças do clube (0x167 bytes em [[0xae2a38]]) +0x166 contador; valor; lê (adiar_pelo_clima)
- reserva do estádio +0 id do estádio, +4 data; valores; lê (ordem_das_reservas_do_estadio)
- Weather [0xde2f74] +0 regiões (0x6e bytes), +4 leituras das cidades (4 bytes: chuva, vento/temperatura, vento da cidade), +8 quantas regiões; lê (clima_da_partida)
- região do clima +0x1e/+0x31/+0x44/+0x57 dias dos períodos, período (19 bytes) +0x20..+0x30 limites; valores; lê (clima_da_partida)
- cidade +0 id, +0x1f nação, +0x34 região do clima; id/ponteiro/valor; lê (clima_da_partida)
- nação +0x7a capital; ponteiro; lê (clima_da_partida)
- jogo +0 id, +0xc/+0x10 valores, +0x18 estádio, +0x1c mandante, +0x2a dia, +0x3f fatia; lê (clima_da_partida, temperatura_do_jogo, gramado_do_jogo)
- estádio +0x38 cidade, +0x3c lugares, +0x4c coberto, +0x4d marca; lê (clima_da_partida, gramado_do_jogo)
- motor +0x4756 clima do jogo (vento, chuva, temperatura, vento da cidade), +1 gramado; grava pela preparação da partida (clima_da_partida, gramado_do_jogo)
- finanças do clube +0/+4 saldo (64 bits), +8 id do clube; valor/id; lê (faixa_de_riqueza)
- clube +0x80 reputação; valor; lê (faixa_de_riqueza)
- dados do jogador +5 habilidade atual, +7 potencial; valores; lê (ordem_por_habilidade)
- local do estádio +0xc estádio; ponteiro; estádio +0x3c lugares; valor; lê (ordem_pela_capacidade)
- TRAINING_MANAGER +0 + 0x9b × vaga: os 167 programas (7 do computador, depois 10 por humano); valor; grava (programas_padrao), lê o endereço (programa_do_humano)
- TRAINING_SCHEDULE +0: vaga (dword com sinal); +4..+8: níveis das cinco categorias (0, 10, 25, 50); +0x15: tipo (−1 a 7); valor; grava (tipo_do_programa)
- pessoa +0: id (os 16 últimos de [0xae2364] são os técnicos humanos; registro de 0x6e bytes em [0xae23bc]); id; lê (programas_padrao, programa_do_humano)
- TRAINING_MANAGER +0x651d: fichas de treino de 6 bytes por ficha de jogador (+0 vaga do programa, 6 goleiros ou 1 geral; +5 bit 0x20); ponteiro e valor; grava (fichas_de_treino_iniciais)
- ficha de jogador (0x46 bytes em [0xae23c4], [0xae236c] fichas) +0: id; +0xf: nota de goleiro (com sinal); id e valor; lê (fichas_de_treino_iniciais)
- pessoa +0x61: ficha de jogador (o id em +0); ponteiro e id; lê (guardar_jogador_no_banco)
- PLAYER_REGEN +0x3c banco de jogadores livres (16 bytes: id da ficha, nação, segunda nação), +0x40 conta, +0x44 capacidade; ponteiro, id e valor; acrescenta (guardar_jogador_no_banco)
- PLAYER_REGEN +0x20: conta da reserva de fichas de não jogador; valor; lê (tem_reserva_de_tecnico)
- jogador da partida +0x2c: tipo do cartão (0 ou 1); +0x2e e +0x2f: código da falta (9 a 12); +0: id; +0x48; valor e id; lê (suspensao_do_vermelho)
- suspensão nova (registro do chamador) +0 jogos, +1, +2 dias, +4 data, +0x10 = 3, +0x1c id do jogador, +0x6f; valor; grava (suspensao_do_vermelho)
- RulingBody +4 vaga e +5 tipo no índice da disciplina; +6 escala; +0xb entradas (14 bytes), +0xf capacidade, +0x13 conta; +0x17 e +0x1b; valor e ponteiro; lê (as três), grava (nova_temporada_do_orgao, compactar_disciplina_do_orgao)
- entrada disciplinar +0 id (−1 livre), +4 e +5 cartões, +6 suspensão (índice com sinal, −1 sem), +8 palavra, +0xd bits; id e valor; lê e grava (nova_temporada_do_orgao, compactar_disciplina_do_orgao)
- [0xae28f0] disciplina: +4 suspensões (20 bytes, +6 ano); vagas guardadas (0x55cb60/0x55cb20, +6 índice); valor; lê e grava (nova_temporada_do_orgao, compactar_disciplina_do_orgao)
- [0xae2c90] data de hoje (+2 ano); valor; lê (suspensao_do_vermelho, nova_temporada_do_orgao)
- Formation +0x12d: a posição geral das 11 casas (palavra, máscara de setor e lado); +0x143 a marca da casa (byte); +0x14e a posição extra (palavra); valor; lê e grava (coordenada_da_casa, posicao_repetida_na_formacao)
- Formation +0x164: as coordenadas das casas (11 × 2 fases × 3 × 4 faixas da bola, dword: x 0..26 e y 0..35); valor; lê e grava (coordenada_da_casa)
- Formation +0x5b4: o tipo de marcação das 11 casas (bits 0..2 e 3..4); +0x187b a máscara da formação; +0x187f zerado a cada mudança; valor; grava (tipo_de_marcacao, coordenada_da_casa)
- [0x9cc368] a tabela das posições (palavras até 0xffff); valor; lê (coordenada_da_casa)
- League +0x3e: a conta dos clubes (palavra); +0x40 a semente da liga (palavra); +0xb1 a tabela (59 bytes: +0 o clube); +0xd9 as marcas (0x100: vagas pela tabela virtual +0xa8/+0xac); +4 a nação; ponteiro e valor; lê e grava a tabela (sortear_ordem_dos_clubes)
- clube +0x69: o estádio (+0x48 o estádio ligado); [0x9cf5dc] a Escócia; [0x9d0118]/[0x9d011c] o Celtic e o Rangers; [0xdd7120] a semente do jogo; ponteiro e id; lê (sortear_ordem_dos_clubes)
- Cup +0x3c: a conta das rodadas (palavra); +0xa7 os clubes; +0xad os jogos de cada rodada (0x35 bytes: +0/+2 os lados, palavra, −1 vazio; +0xe/+0xa e +0x10/+0xc a rodada e o jogo de onde vem cada lado); [0xae1880] a conta; valor e ponteiro; lê e grava a conta (clubes_do_lado_do_jogo, clubes_possiveis_do_jogo)
- clube +0x77: o dinheiro (dword); +0x80 a reputação (palavra); +0x57 a nação (+0x69 a reputação, palavra); +0x1db a liga em [0xae23d0] (0x6b bytes, +0x69 a reputação); +0x64 profissional (byte, 1); ponteiro e valor; lê (nota_alvo_do_clube)
- [0xadadfc] as nações com liga (dword por id); [0xae2a38] as finanças; ponteiro; lê (nota_alvo_do_clube)
- jogador acompanhado +6: o anel de 10 jogos (0x29 bytes, +0x16 os gols, byte com sinal); +0x1a0 o começo; +0x1a1 a conta; valor; lê (jogo_do_historico, gols_do_historico)

**Interessa a:** CMRTE, CMScout, SGE, GPF2
