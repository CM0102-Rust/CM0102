# 277ª leva — as regras de transferência da Noruega, da Irlanda do Norte e do País de Gales: as janelas de transferência; as regras de transferência da Turquia, da Polônia e da Finlândia: as janelas de transferência; as regras de transferência da Argentina, de Portugal e da Coreia do Sul: as janelas de transferência; as regras de transferência da Grécia, da Rússia e da Bélgica: as janelas de transferência; a inscrição das três copas europeias, a tabela inicial do ranking da FIFA e a lista dos estádios neutros; a tabela das três divisões nacionais do Brasil: os clubes da Série A e da Série B e o sorteio da Série C; a tabela dos estaduais: Baiano, Centro-Oeste e Gaúcho; a tabela dos estaduais: Goiano, Mineiro e Norte; a tabela dos estaduais: Nordeste, Paranaense e Pernambucano; a tabela dos estaduais: Carioca, Catarinense e Paulista; a tabela das ligas do Japão e da Austrália: os clubes da J1, da J2 e da NSL, o fundo de prêmios e a TV da J1; as semifinais dos estaduais e regionais: Baiano, Norte e Nordeste; as semifinais dos estaduais e regionais: Pernambucano, Catarinense e Paulista

**Rotinas:**
- `regras_noruega()` = 0x7944f0 (norway_rules.cpp, thiscall, ret 0x14): a regra da Noruega (17): uma janela, de 1º de setembro a 31 de agosto;
- `regras_irlanda_do_norte()` = 0x7936d0 (northern_ireland_rules.cpp, thiscall, ret 0x14): a regra da Irlanda do Norte (16): uma janela, de 6 de maio (andando para o sábado [inferência]) a 20 de março;
- `regras_pais_de_gales()` = 0x90f080 (wales_rules.cpp, thiscall, ret 0x14): a regra do País de Gales (26): uma janela, de 7 de maio (andando para o domingo [inferência]) a 20 de março;
- `regras_turquia()` = 0x9020a0 (turkey_rules.cpp, thiscall, ret 0x14): a regra da Turquia (24): uma janela, de 1º de junho a 31 de janeiro; o +0x13 = 8 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 8 (o padrão é 30);
- `regras_polonia()` = 0x7cd830 (poland_rules.cpp, thiscall, ret 0x14): a regra da Polônia (18): uma janela, de 1º de julho a 20 de fevereiro; o +0x13 = 5 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 5 (o padrão é 30);
- `regras_finlandia()` = 0x5a9950 (finland_rules.cpp, thiscall, ret 0x14): a regra da Finlândia (8): uma janela, de 16 de novembro a 15 de agosto; o dword +0x15 = 1;
- `janelas_argentina()` = 0x40a620 (argentina_rules.cpp, thiscall, ret 0): a regra da Argentina (1): duas janelas, de 24 de julho (andando para o domingo [inferência]) a 6 de agosto (andando para a quarta-feira [inferência]); de 14 de dezembro (andando para o domingo [inferência]) a 4 de março (andando para a quarta-feira [inferência]);
- `janelas_portugal()` = 0x7d3df0 (portugal_rules.cpp, thiscall, ret 0): a regra de Portugal (19): duas janelas, de 1º de junho a 1º de setembro; de 15 de dezembro a 15 de janeiro;
- `regras_coreia()` = 0x66e630 (korea_rules.cpp, thiscall, ret 0x14): a regra da Coreia do Sul (27): uma janela, de 25 de novembro (andando para o domingo [inferência]) a 25 de julho; o +0x13 = 7 (o padrão da base é 50) [inferência: o limite de estrangeiros];
- `janelas_grecia()` = 0x5ecf20 (greece_rules.cpp, thiscall, ret 0): a regra da Grécia (11): duas janelas, de 22 de maio (andando para o domingo [inferência]) a 29 de agosto (andando para o sábado [inferência]); de 1º de janeiro a 20 de janeiro;
- `janelas_russia()` = 0x7ec7e0 (russia_rules.cpp, thiscall, ret 0): a regra da Rússia (20): duas janelas, de 15 de dezembro a 24 de março (andando para a quinta-feira [inferência]); de 28 de junho (andando para a terça-feira [inferência]) a 4 de agosto;
- `janelas_belgica()` = 0x424c00 (belgium_rules.cpp, thiscall, ret 0): a regra da Bélgica (3): três janelas, de 1º de junho a 30 de junho; de 1º de junho a 31 de dezembro; de 1º de junho a 31 de março;
- `inscricao_europeia()` = 0x85a9b0 (squad_manager.cpp, thiscall, ret 4): a regra: na primeira etapa as inscrições de todos são zeradas; em cada etapa, cada clube que joga a Liga dos Campeões, a Copa da UEFA ou a terceira copa [inferência: a Recopa] escolhe os inscritos (os clubes de técnico humano recebem a notícia e escolhem eles); na carreira, a etapa 3 roda em 2002 (fora de 2001);
- `ranking_fifa_inicial()` = 0x58dd50 (fifa_rankings.cpp, thiscall, ret 0): a regra do ranking inicial da FIFA: os pontos de cada nação são a força dela no banco ÷ 35, repetidos como os pontos de cada um dos 6 anos [inferência] (× 1000); a posição sai da ordem dos pontos;
- `lista_de_estadios_neutros()` = 0x85dfc0 (stadium.cpp, thiscall, ret 0): a regra dos campos neutros (finais e semifinais): servem os estádios com cidade, nação e continente, menos o City of Manchester Stadium (em obras) e, nas nações com competições, os de menos de 5000 lugares; o estádio nacional de cada nação fica marcado;
- `cql7_tabela_da_serie_a()` = 0x42a480 (bra_nat_first.cpp, thiscall, ret 0): a Série A tem 28 clubes: os da divisão nacional da Série A, na ordem do banco; cheia, a TV paga 150.000 a cada clube (sem fundo de prêmios);
- `cql7_tabela_da_serie_b()` = 0x42ca70 (bra_nat_second.cpp, thiscall, ret 0): a Série B tem 28 clubes, os da divisão na ordem do banco; sem dinheiro da TV nem fundo de prêmios;
- `cql7_tabela_da_serie_c()` = 0x42dc90 (bra_nat_third.cpp, thiscall, ret 0): a Série C recebe até 56 clubes (os da divisão, na ordem do banco), que são sorteados (3 trocas por clube); os 7 primeiros do sorteio formam a primeira tabela, de 7 clubes, e a lista sorteada fica guardada para os outros grupos [inferência: 8 grupos de 7 na primeira fase];
- `cql8_tabela_do_baiano()` = 0x42fd10 (bra_reg_bahia.cpp, thiscall, ret 0): O Baiano tem 10 clubes: os que têm Baiano como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cql8_tabela_do_centro_oeste()` = 0x430ef0 (bra_reg_central.cpp, thiscall, ret 0): O Centro-Oeste tem 15 clubes: os que têm Centro-Oeste como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cql8_tabela_do_gaucho()` = 0x4321d0 (bra_reg_gaucho.cpp, thiscall, ret 0): O Gaúcho tem 17 clubes: os que têm Gaúcho como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cql9_tabela_do_goiano()` = 0x433540 (bra_reg_goias.cpp, thiscall, ret 0): O Goiano tem 14 clubes: os que têm Goiano como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cql9_tabela_do_mineiro()` = 0x434810 (bra_reg_minas_gerais.cpp, thiscall, ret 0): O Mineiro tem 12 clubes: os que têm Mineiro como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cql9_tabela_do_norte()` = 0x435d80 (bra_reg_north.cpp, thiscall, ret 0): O Norte tem 10 clubes: os que têm Norte como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cqm7_tabela_do_nordeste()` = 0x436d90 (bra_reg_northeast.cpp, thiscall, ret 0): O Nordeste tem 16 clubes: os que têm Nordeste como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cqm7_tabela_do_paranaense()` = 0x437ec0 (bra_reg_parana.cpp, thiscall, ret 0): O Paranaense tem 12 clubes: os que têm Paranaense como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cqm7_tabela_do_pernambucano()` = 0x439330 (bra_reg_pern.cpp, thiscall, ret 0): O Pernambucano tem 10 clubes: os que têm Pernambucano como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cqm8_tabela_do_carioca()` = 0x43a500 (bra_reg_rio.cpp, thiscall, ret 0): O Carioca tem 12 clubes: os que têm Carioca como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cqm8_tabela_do_catarinense()` = 0x43bb70 (bra_reg_santa.cpp, thiscall, ret 0): O Catarinense tem 10 clubes: os que têm Catarinense como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cqm8_tabela_do_paulista()` = 0x43cc40 (bra_reg_sp.cpp, thiscall, ret 0): O Paulista tem 16 clubes: os que têm Paulista como divisão reserva no banco (o campo que guarda o estadual ou o regional), na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios;
- `cqm9_tabela_da_j1()` = 0x6634e0 (jap_j1.cpp, thiscall, ret 0): a J1 tem 16 clubes, os da divisão na ordem do banco; os que passam de 16 perdem a divisão (ficam sem liga); cheia, o fundo de prêmios parte de 100.000 por posição e a TV paga 3.000.000 a cada clube;
- `cqm9_tabela_da_j2()` = 0x665030 (jap_j2.cpp, thiscall, ret 0): a J2 tem 12 clubes, os da divisão na ordem do banco; os que passam de 12 perdem a divisão; sem fundo de prêmios nem TV;
- `cqm9_tabela_da_nsl()` = 0x411b70 (aus_nsl.cpp, thiscall, ret 0): a NSL australiana tem 13 clubes, os da divisão na ordem do banco; com menos, o jogo mostra a caixa de erro; sem fundo de prêmios nem TV;
- `cqn7_mata_mata_do_baiano()` = 0x430050 (bra_reg_bahia.cpp, thiscall, ret 0): O Baiano: depois dos pontos corridos, os 4 primeiros vão às semifinais: 1º × 4º, 2º × 3º (o mais bem colocado é a semente; a ordem dos jogos segue as sementes [inferência: ida e volta, e a decisão em casa do mais bem colocado]);
- `cqn7_mata_mata_do_norte()` = 0x4360c0 (bra_reg_north.cpp, thiscall, ret 0): O Norte: depois dos pontos corridos, os 4 primeiros vão às semifinais: 1º × 4º, 2º × 3º (o mais bem colocado é a semente; a ordem dos jogos segue as sementes [inferência: ida e volta, e a decisão em casa do mais bem colocado]);
- `cqn7_mata_mata_do_nordeste()` = 0x4370d0 (bra_reg_northeast.cpp, thiscall, ret 0): O Nordeste: depois dos pontos corridos, os 4 primeiros vão às semifinais: 1º × 4º, 2º × 3º (o mais bem colocado é a semente; a ordem dos jogos segue as sementes [inferência: ida e volta, e a decisão em casa do mais bem colocado]);
- `cqn8_mata_mata_do_pernambucano()` = 0x439710 (bra_reg_pern.cpp, thiscall, ret 0): O Pernambucano: depois dos pontos corridos, os 4 primeiros vão às semifinais: 1º × 4º, 2º × 3º (o mais bem colocado é a semente; a ordem dos jogos segue as sementes [inferência: ida e volta, e a decisão em casa do mais bem colocado]);
- `cqn8_mata_mata_do_catarinense()` = 0x43bf50 (bra_reg_santa.cpp, thiscall, ret 0): O Catarinense: depois dos pontos corridos, os 4 primeiros vão às semifinais: 1º × 4º, 2º × 3º (o mais bem colocado é a semente; a ordem dos jogos segue as sementes [inferência: ida e volta, e a decisão em casa do mais bem colocado]);
- `cqn8_mata_mata_do_paulista()` = 0x43cf80 (bra_reg_sp.cpp, thiscall, ret 0): O Paulista: depois dos pontos corridos, os 4 primeiros vão às semifinais: 1º × 4º, 2º × 3º (o mais bem colocado é a semente; a ordem dos jogos segue as sementes [inferência: ida e volta, e a decisão em casa do mais bem colocado]);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (39):** 39 acusadas.
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 1dedd9dc, original 1e6682cb (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 276ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 277`

**Campos tocados:**
- regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)
- regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)
- regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)
- regras de transferência +0 vtable, +4 janelas (datas de 6 bytes, duas por janela), +8 número de janelas, +9 a regra, +0xa a nação, +0xe o continente, +0x12, +0x13 (padrão 50) [inferência: limite de estrangeiros], +0x14 (padrão 30), +0x15; ponteiro e valor; grava (todas)
- pessoa (registros de 0x6e bytes em [0xae23bc]) +0x6d marca de inscrição europeia; valor; grava 0 na etapa 1 (inscricao_europeia)
- clube +0x1db competição europeia; id; lê (inscricao_europeia); SQUAD_MANAGER +0 ano das inscrições; valor; grava (inscricao_europeia)
- nação (0x122 bytes em [0xae23a8]) +0xa8 força (double); valor; lê (ranking_fifa_inicial); FIFA_RANKINGS +0 a tabela (0x2c bytes por nação: +0 índice, +0xc pontos float, +0x10 posição, +0x12 posição anterior, +0x14..+0x28 6 dwords de pontos); ponteiro e valor; grava (ranking_fifa_inicial)
- estádio (0x4e bytes em [0xae23b0], conta [0xae2358]) +4 nome, +0x38 cidade, +0x3c lugares; cidade +0x1f nação; nação +0x71 continente, +0x80 estádio nacional, +0x11c tem competições; valor e ponteiro; lê (lista_de_estadios_neutros)
- Stadium +0xc a lista dos campos neutros (16 bytes: nação, continente, nacional, estádio), +0x10 conta, +0x14 capacidade, +0x18 pronta; ponteiro e valor; grava (lista_de_estadios_neutros)
- clube +0x57: divisão nacional (registro de competição); ponteiro; lê (as três)
- [0x9cf9ec] / [0x9cf9e0]: id da Série A / da Série B; id; lê
- liga +4: o registro da própria competição; ponteiro; lê (Série C)
- liga +0x3e: número de times da tabela; valor; grava (28, 28, 7)
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; grava
- Série C +0xee: a lista sorteada dos participantes (56 ponteiros); ponteiro; libera e grava
- clube +0x60: divisão reserva (ReserveDivision; no Brasil, o estadual); ponteiro; lê
- [0x9cf9c0] / [0x9cf998] / [0x9cf9c8]: ids do Baiano, do Centro-Oeste, do Gaúcho; id; lê
- liga +0x3e: número de times da tabela; valor; grava
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; grava
- clube +0x60: divisão reserva (ReserveDivision; no Brasil, o estadual); ponteiro; lê
- [0x9cf9b4] / [0x9cf9d4] / [0x9cf994]: ids do Goiano, do Mineiro, do Norte; id; lê
- liga +0x3e: número de times da tabela; valor; grava
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; grava
- clube +0x60: divisão reserva (ReserveDivision; no Brasil, o estadual); ponteiro; lê
- [0x9cf9a4] / [0x9cf9d0] / [0x9cf9bc]: ids do Nordeste, do Paranaense, do Pernambucano; id; lê
- liga +0x3e: número de times da tabela; valor; grava
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; grava
- clube +0x60: divisão reserva (ReserveDivision; no Brasil, o estadual); ponteiro; lê
- [0x9cf9dc] / [0x9cf9b0] / [0x9cf9e4]: ids do Carioca, do Catarinense, do Paulista; id; lê
- liga +0x3e: número de times da tabela; valor; grava
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; grava
- clube +0x57: divisão (registro de competição); ponteiro; lê (as três) e grava 0 nos que sobram (J1, J2)
- [0x9cf65c] / [0x9cf768] / [0x9cf89c]: ids da J1, da J2 e da NSL; id; lê
- liga +0x3e: número de times da tabela; valor; grava (16, 12, 13)
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; grava
- liga +0x28: fundo de prêmios; valor; grava (J1)
- liga +0xe1: marca da tabela pronta; valor; grava 1 (J1)
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê
- liga +0xc: as fases (vetor de ponteiros); ponteiro; grava a primeira
- liga +0x30: a fase atual; valor; grava 0
- liga +0x40: o ano da temporada; valor; lê
- fase de copa +0xa7: a lista dos times (6 bytes, +0 o clube); ponteiro; lê (comparador)
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê
- liga +0xc: as fases (vetor de ponteiros); ponteiro; grava a primeira
- liga +0x30: a fase atual; valor; grava 0
- liga +0x40: o ano da temporada; valor; lê
- fase de copa +0xa7: a lista dos times (6 bytes, +0 o clube); ponteiro; lê (comparador)

**Interessa a:** CMRTE, SGE
