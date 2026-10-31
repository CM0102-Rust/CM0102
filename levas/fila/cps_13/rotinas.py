# O dia a dia das copas e o risco de doença na viagem (Cup.cpp, fix_man.cpp): Cup::update_for_date (0x51f890, a
# atualização diária de cada copa, 29.934 chamadas na cobertura de 17/10: fases completas, fim da copa, o dia das
# fases, os sorteios e as montagens das rodadas nas datas), Cup::set_up_round (0x51c800, 68: sorteia se falta, cria os
# jogos dos confrontos pela 0x51cb90, leva os jogos às datas da rodada pela 0x51ebe0 e passa a rodada; time ainda sem
# clube adia a rodada um dia) e FIXTURE_MANAGER::add_away_trip_illness_risk (0x5ab3c0, 6.064: o risco de doença na
# véspera para os visitantes de um país do terceiro mundo, pela 0x62e820). O SEH das três só cobre datas e o bloco da
# data dos jogos (sem destrutor a reproduzir). Sem leitura de local sem valor (as datas comparadas são as montadas pela
# 0x549ef0 antes). Os dwords empurrados à 0x549ef0 levam na metade alta a dos ponteiros que estavam nos registradores
# (rodada, tabela de times, clube, nação, estádio, cidade): a cópia reproduz; a 0x549ef0 lê as palavras. Iguais no
# cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Comparadores: as três
# rodam só a original (sorteiam, alocam jogos, acrescentam o risco); a cópia leva os ramos fora do caminho comum.
SAVE = False
ROTINAS = [('atualizar_copa_no_dia', 0x51f890, [0x6a, 0xff, 0x68, 0x98, 0x77, 0x95, 0x00]),
           ('montar_rodada_da_copa', 0x51c800, [0x6a, 0xff, 0x68, 0xd9, 0x76, 0x95, 0x00]),
           ('risco_de_doenca_na_viagem', 0x5ab3c0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o dia a dia das copas e o risco de doença na viagem: a atualização da copa no dia, montar a rodada da copa e o risco de doença dos visitantes',
        'the cups day by day and the travel illness risk: the cup daily update, setting up the cup round and the visitors\' illness risk',
        [('atualizar_copa_no_dia', 'Cup::update_for_date', '0x51f890', 'Cup.cpp', 'thiscall, ret 8',
          'passa as fases completas e encerra a copa, dá o dia a cada fase e, na data, sorteia (fase 0) ou monta (fase ≠ 0) as rodadas até não avançar',
          'moves on completed stages and ends the cup, gives each stage the day and, on the date, draws (phase 0) or sets up (phase ≠ 0) the rounds until none advances'),
         ('montar_rodada_da_copa', 'Cup::set_up_round', '0x51c800', 'Cup.cpp', 'thiscall, ret 4',
          'sorteia a rodada se falta, cria os jogos dos confrontos marcados, leva-os às datas da rodada e passa à seguinte; time sem clube adia as rodadas um dia',
          'draws the round if needed, creates the fixtures of the marked ties, moves them to the round dates and moves on; a team without a club postpones the rounds by a day'),
         ('risco_de_doenca_na_viagem', 'FIXTURE_MANAGER::add_away_trip_illness_risk', '0x5ab3c0', 'fix_man.cpp', 'thiscall, ret 4',
          'com o estádio num país do terceiro mundo, dá aos clubes de fora o risco de doença na véspera do jogo',
          'with the stadium in a third-world country, gives the visiting clubs the illness risk on the day before the fixture')])
CAMPOS = [('copa +0x3c: as rodadas (palavra); +0x40 o ano-base; +0x44 o pai (0xff sem); +0x45 ativa; +0x4c encerrada; +0x2c/+0x30 as fases e a atual; +0xc as fases; +8 o Comp_Stats; +0xb1 com fases; valor e ponteiro; lê e grava +0x4c (atualizar_copa_no_dia)',
           'cup +0x3c: rounds (word); +0x40 base year; +0x44 parent (0xff none); +0x45 active; +0x4c finished; +0x2c/+0x30 stages and the current one; +0xc the stages; +8 the Comp_Stats; +0xb1 has stages; value and pointer; reads and writes +0x4c (atualizar_copa_no_dia)'),
          ('copa +0xa3: as rodadas (0x68 bytes: +0 o dia e +2 o ano do sorteio, +3/+5 a data dos jogos, +7 o nome, 0x8c a final, +0x1a os confrontos); +0xa7 os times (6 bytes, +0 o clube); +0xab a rodada atual; +0xac a do sorteio; +0xad as fases das rodadas (+0x34 pendente; confrontos de 0x35 bytes, +0/+2 os times, +0x34 a marca); lê e grava (as duas da copa)',
           'cup +0xa3: the rounds (0x68 bytes: +0 draw day and +2 year, +3/+5 the fixtures date, +7 the name, 0x8c the final, +0x1a the ties); +0xa7 the teams (6 bytes, +0 the club); +0xab current round; +0xac draw round; +0xad round stages (+0x34 pending; 0x35-byte ties, +0/+2 the teams, +0x34 the mark); reads and writes (both cup ones)'),
          ('[0xadadfc][id da competição]: a competição ativa; [0xae2c88] o modo do dia; valor; lê (as duas da copa)',
           '[0xadadfc][competition id]: the active competition; [0xae2c88] the day mode; value; reads (both cup ones)'),
          ('jogo +0x18 o estádio (+0x38 a cidade, +0x1f a nação, +0x7e o desenvolvimento); +0x1c/+0x20 os clubes (+0x53 a nação); +0xc/+0x10 os ids; +0x28/+0x2a o ano e o dia; ponteiro e valor; lê (risco_de_doenca_na_viagem)',
           'fixture +0x18 the stadium (+0x38 the city, +0x1f the nation, +0x7e state of development); +0x1c/+0x20 the clubs (+0x53 the nation); +0xc/+0x10 the ids; +0x28/+0x2a year and day; pointer and value; reads (risco_de_doenca_na_viagem)')]
INTERESSA = ['CMRTE']
