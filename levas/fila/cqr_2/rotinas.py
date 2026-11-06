# A virada da temporada do Carioca, do Mineiro e do Paranaense (o slot start_new_season de bra_reg_rio.cpp,
# bra_reg_minas_gerais.cpp e bra_reg_parana.cpp; agente C da frente das competições, 06/11/2026). Os três estaduais de
# dois turnos e final; o mesmo molde de 464 bytes (conferido instrução a instrução: só mudam o fonte e a rotina da
# tabela). Cada um roda uma vez na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt), na virada do
# Brasil para 2002 (chamados pela virada da Série A, cqq_8).
# - 0x43a6a0 BraRegRio::start_new_season, 0x434910 BraRegMinasGerais::start_new_season,
#   0x438060 BraRegParana::start_new_season.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - os três, da virada da Série A: a virtual +8 (mov ecx, liga; call [edx+8], sem push) -> thiscall, ret 0;
# - 0x687970 (free_table): push 0; mov ecx, esi -> thiscall, ret 4; 0x9452ca (free): push; call; add esp -> cdecl;
# - 0x4a1c50 (archive_season_and_reset): push 1; mov ecx, [esi+8] -> thiscall, ret 4;
# - o destrutor das fases (virtual 0): push 1; mov ecx, fase -> thiscall, ret 4;
# - 0x944e46 (malloc): push 0xc; call; add esp (junto com o do free) -> cdecl;
# - as virtuais +0x8c (regras) e +0x5c (posições previstas): mov ecx, esi, sem push -> thiscall, ret 0;
# - as tabelas 0x43a500, 0x434810, 0x437ec0 e 0x6835c0 (create_fixtures): mov ecx, esi; call -> thiscall, ret 0;
#   0x6827d0 (sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x79cee0 (renew_officials): push [esi+4]; mov ecx, [0xb74340] -> thiscall, ret 4;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original (libera e aloca, sorteia jogos e árbitros), conferindo o ano + 1, +0x2c = 3 e +0x30 = −1.
# Determinismo: nada de pilha, heap ou endereço como valor (o vetor novo das fases fica sem zerar, como na original:
# os turnos o preenchem antes de lê-lo).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr2_virada_do_carioca', 0x43a6a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqr2_virada_do_mineiro', 0x434910, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqr2_virada_do_paranaense', 0x438060, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
_V = ('A virada {liga}: guarda a temporada nas estatísticas, apaga a tabela, o calendário e as fases (os dois turnos e a final) e prepara três vagas novas de fase; passa ao ano seguinte e refaz as regras, a tabela com os doze clubes do estadual, os jogos e a ordem; depois as posições previstas e a renovação dos árbitros. Os turnos só são montados no primeiro jogo de cada um (cqr_1)',
      'The {liga_en} turnover: archives the season in the statistics, clears the table, the schedule and the stages (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds the rules, the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the referee renewal. The halves are only built at each one\'s first game (cqr_1)')
def _v(liga, liga_en):
    return (_V[0].format(liga=liga), _V[1].format(liga_en=liga_en))
DOCS = ('a virada da temporada do Carioca, do Mineiro e do Paranaense',
        'the season turnover of the Rio, Minas Gerais and Parana state leagues',
        [('cqr2_virada_do_carioca', 'BraRegRio::start_new_season', '0x43a6a0', 'bra_reg_rio.cpp', 'thiscall, ret 0') + _v('do Carioca', 'Rio state league'),
         ('cqr2_virada_do_mineiro', 'BraRegMinasGerais::start_new_season', '0x434910', 'bra_reg_minas_gerais.cpp', 'thiscall, ret 0') + _v('do Mineiro', 'Minas Gerais state league'),
         ('cqr2_virada_do_paranaense', 'BraRegParana::start_new_season', '0x438060', 'bra_reg_parana.cpp', 'thiscall, ret 0') + _v('do Paranaense', 'Parana state league')])
CAMPOS = [('liga +0x4c: [inferência] marca da temporada; valor; grava 0', 'league +0x4c: [inference] season flag; value; writes 0'),
          ('liga +0xba: o calendário (rodadas de 0x41 bytes); ponteiro; libera e zera', 'league +0xba: the schedule (0x41-byte rounds); pointer; frees and clears'),
          ('liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê', 'league +8: the statistics block (Comp_Stats); pointer; reads'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; destrói as fases, libera e grava um vetor novo de 3', 'league +0xc: the stages (pointer array); pointer; destroys the stages, frees and writes a new 3-slot array'),
          ('liga +0x2c: a quantidade de fases; valor; grava 3', 'league +0x2c: the stage count; value; writes 3'),
          ('liga +0x30: a fase atual; valor; lê e grava −1', 'league +0x30: the current stage; value; reads and writes −1'),
          ('liga +0x40: o ano da temporada; valor; lê e grava + 1', 'league +0x40: the season year; value; reads and writes + 1'),
          ('liga +4: o registro da competição no banco; ponteiro; lê (árbitros)', 'league +4: the competition record in the database; pointer; reads (referees)'),
          ('[0xb74340] o gerente dos árbitros; ponteiro; lê', '[0xb74340] the officials manager; pointer; reads')]
INTERESSA = []
