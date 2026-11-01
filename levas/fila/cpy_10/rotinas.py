# O prêmio do ano no histórico (staff_records.cpp): Staff_Records::add_year_award_record 0x86c390, pelo registro dos
# vencedores do prêmio do ano YEAR_AWARD::record_winners 0x9302f0 (o primeiro, o segundo e o terceiro de cada prêmio
# do ano: os das ligas que terminam antes de 2002-06-01 e os de dezembro). É a irmã da 0x86c8c0 (prêmio do mês, 258ª):
# a entrada 0x34 em vez de 0x33, os campos a4 (palavra), a5 e o crédito (bytes com sinal) e o sorteio do agradecimento
# só com o crédito 1. Usa os ajudantes da 253ª e da 258ª (cpq3_renovar, cpq4_: entrada, data, arquivo FILE_LLIST,
# registro de pontos 0x86f5b0; cpv13_clube_do_contrato). Quadro SEH sem destrutor. Passageiras pelos recordes do clube
# depois de cada jogo (Club_Records::update_records_for_match 0x444930; record_utils.cpp): match_record_is_bigger_win
# 0x7e4a30 e match_record_is_bigger_defeat 0x7e4b00 (os recordes do clube não entram na carreira: as mutações ficam na
# 0x86c390). Iguais no cm0102-1080.exe. Comparadores: o prêmio do técnico lado a lado pelo registro de pontos dele (a
# cópia sem o arquivo do histórico); o do jogador só pela original (sorteio, renovação e notícia); as duas do recorde
# puras (o registro nulo, a caixa, só a original).
# Convenções conferidas nos pontos de chamada das rotinas chamadas por endereço (0x86c390):
# - 0x54a110 data_padrao e 0x417c10 copia: `lea ecx, [esp+0xc]; call` e `push 0xae2c90; push eax; lea ecx; call` sem
#   ajuste = thiscall (o cpq4_entrada e o cpq4_datar do motor.rs).
# - 0x58e340 (FILE_LLIST): `push ecx; push edx; push eax; mov ecx, [edi]; call` sem ajuste = thiscall(arquivo, começo,
#   fim, entrada) (cpq4_historico).
# - 0x86f5b0 registro_do_tecnico: `push 1; push esi; mov ecx, edi; call` = thiscall(this, pessoa, 1) (cpq4_registro).
# - 0x4cf5a0 contrato_de: `mov ecx, [0xae1050]; push esi; call` = thiscall(gerente, pessoa) (0x86c4fc, 0x86c542,
#   0x86c5b9).
# - 0x4cf5f0 contrato_com_clube: `mov ecx, [0xae1050]; push eax; push esi; call` = thiscall(gerente, pessoa, clube).
# - 0x54cdb0 dias_ate: `push eax; push ecx; mov ecx, 0xae2c90; call` = thiscall(data, 2 dwords da data).
# - 0x849ce0 entrada_da_lista: `push 0; push 0; push esi; mov ecx, [ecx+0x856]; push eax; call` sem ajuste =
#   thiscall(lista, clube, pessoa, 0, 0).
# - 0x4cda70 (oferta de renovação): `mov ecx, [0xae1050]` e 7 push sem ajuste = thiscall(gerente, pessoa, 0, 0, −1, 0,
#   0x3ff00000, 1) (cpq3_renovar).
# - 0x90cfc0 sortear: `push eax; call; add esp, 4` = cdecl(n).
# - 0x58a070 (notícia do agradecimento): `push ecx; push edx; push esi; call; add esp, 0xc` = cdecl(pessoa, clube,
#   prêmio).
# - Os pontos de chamada da 0x86c390 (0x930389, 0x930423, 0x9304bd) com ecx e 6 push (ret 0x18); os da 0x7e4a30 e da
#   0x7e4b00 com 2 push e `add esp, 8` (cdecl).
SAVE = False
ROTINAS = [('registrar_premio_do_ano', 0x86c390, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('vitoria_maior', 0x7e4a30, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('derrota_maior', 0x7e4b00, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o prêmio do ano no histórico (o técnico conta, o jogador premiado agradece ou renova) e os recordes de vitória e de derrota',
        'the year award in the history (the manager counts it, the awarded player thanks or renews) and the win and defeat records',
        [('registrar_premio_do_ano', 'Staff_Records::add_year_award_record', '0x86c390', 'staff_records.cpp', 'thiscall, ret 0x18',
          'a entrada do prêmio do ano no histórico da pessoa; o técnico soma o prêmio no registro dele; o jogador com contrato longo na lista do clube recebe a oferta de renovação e o primeiro colocado pode agradecer ao técnico (sorteio pela lealdade)',
          'the year award entry in the person history; the manager adds the award to his record; the player with a long contract on the club list gets the renewal offer and the winner may thank the manager (draw by loyalty)'),
         ('vitoria_maior', 'match_record_is_bigger_win', '0x7e4a30', 'record_utils.cpp', 'cdecl',
          'o registro de partida é uma vitória por diferença maior que a do outro, ou igual com mais gols feitos',
          'the match record is a win by a larger margin than the other one, or the same with more goals scored'),
         ('derrota_maior', 'match_record_is_bigger_defeat', '0x7e4b00', 'record_utils.cpp', 'cdecl',
          'o registro de partida é uma derrota por diferença maior que a do outro, ou igual com mais gols sofridos',
          'the match record is a defeat by a larger margin than the other one, or the same with more goals conceded')])
CAMPOS = [('entrada do histórico da pessoa (FILE_LLIST [Staff_Records], começo e fim em [+0xc] + id × 8) +0 data, +8 clube ou −1, +0xc tipo 0x34, +0xd prêmio, +0x11 a4, +0x15 a5, +0x19 crédito; valores; grava (registrar_premio_do_ano)',
           'person history entry (FILE_LLIST [Staff_Records], head and tail at [+0xc] + id × 8) +0 date, +8 club or −1, +0xc type 0x34, +0xd award, +0x11 a4, +0x15 a5, +0x19 credit; values; writes (registrar_premio_do_ano)'),
          ('registro de pontos do técnico (0x86f5b0) +0x9e prêmios (palavra); valor; lê e grava (registrar_premio_do_ano)',
           'manager points record (0x86f5b0) +0x9e awards (word); value; reads and writes (registrar_premio_do_ano)'),
          ('pessoa +0 id, +0x28 e +0x3d cargos, +0x59 e +0x5b atributos de caráter (bytes com sinal), +0x61 ficha de jogador; contrato (0x4cf5a0/0x4cf5f0) +4 clube, +0x25/+0x29 data do fim, +0x35 tipo, +0x45 marca; [0xae1080] último contrato; valores/ponteiros; lê, grava [0xae1080] (registrar_premio_do_ano)',
           'person +0 id, +0x28 and +0x3d jobs, +0x59 and +0x5b character attributes (signed bytes), +0x61 player record; contract (0x4cf5a0/0x4cf5f0) +4 club, +0x25/+0x29 end date, +0x35 type, +0x45 flag; [0xae1080] last contract; values/pointers; reads, writes [0xae1080] (registrar_premio_do_ano)'),
          ('registro de partida +0x19 gols feitos, +0x1a gols sofridos (bytes com sinal); valores; lê (vitoria_maior, derrota_maior)',
           'match record +0x19 goals for, +0x1a goals against (signed bytes); values; reads (vitoria_maior, derrota_maior)')]
INTERESSA = ['CMRTE']
