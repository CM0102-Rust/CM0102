# Os convocados por clube (national_teams.cpp) e duas da competição (comp.cpp): NationalTeams::record_club_call_ups
# (0x7771c0, 684 chamadas na cobertura de 17/10: a cada convocação para um jogo de seleção), a importância do jogo
# pela fase (0x48e190, virtual da competição, passageira) e a fase da competição (0x48d200, passageira). Os registros
# que a 0x7771c0 grava são os que a 0x776fc0 (já em Rust) lê para não marcar jogo de clube com mais de dois convocados
# perto da data (o "cpp_12 called up in the window" conta 64 na base da 257ª). A 0x7771c0 chama rotinas já em Rust (datas,
# 0x5409b0, os qsort com 0x778ef0 e 0x778e90) e o realloc da CRT (0x945667); quadro SEH só do estado. Iguais no
# cm0102-1080.exe.
# Comparadores: a 0x7771c0 lado a lado (elenco do time, registros e vetores dos clubes do elenco) quando nenhum vetor
# pode crescer; senão só a original, com o elenco ordenado conferido; as duas de comp.cpp pelo valor.
SAVE = False
ROTINAS = [('registrar_convocados_por_clube', 0x7771c0, [0x6a, 0xff, 0x68, 0x6b, 0xdf, 0x95, 0x00]),
           ('importancia_do_jogo_na_competicao', 0x48e190, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('fase_da_competicao', 0x48d200, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os convocados por clube na data do jogo da seleção, a importância do jogo e a fase da competição',
        "the call-ups per club on the national side's match date, the match importance and the competition stage",
        [('registrar_convocados_por_clube', 'NationalTeams::record_club_call_ups', '0x7771c0', 'national_teams.cpp', 'thiscall, ret 0xc',
          'ordena o elenco da seleção pelo clube e soma, na data do jogo, quantos convocados cada clube cede (o registro que impede marcar jogo do clube perto da data)',
          'sorts the national squad by club and adds, on the match date, how many players each club gives up (the record that stops club fixtures near that date)'),
         ('importancia_do_jogo_na_competicao', 'Comp::get_fixture_match_importance', '0x48e190', 'comp.cpp', 'thiscall, ret 0xc',
          'repassa a pergunta da importância do jogo à fase dele (virtual +0x54)',
          "forwards the match importance query to the fixture's stage (virtual +0x54)"),
         ('fase_da_competicao', 'Comp::get_stage', '0x48d200', 'comp.cpp', 'thiscall, ret 4',
          'devolve a fase k da competição (−1 = ela mesma); fora da faixa, a caixa e 0',
          'returns stage k of the competition (-1 = itself); out of range, the error box and 0')])
CAMPOS = [('time da seleção +0xd7: o elenco (50 vagas); ponteiro; lê e reordena pelo clube (registrar_convocados_por_clube)',
           "national side +0xd7: the squad (50 slots); pointer; reads and reorders by club (registrar_convocados_por_clube)"),
          ('staff +0x39: o clube; ponteiro; lê; +0x5e bits 2 e 3: o elenco B/principal; valor; lê (registrar_convocados_por_clube)',
           'staff +0x39: the club; pointer; reads; +0x5e bits 2 and 3: B/main squad; value; reads (registrar_convocados_por_clube)'),
          ('[NationalTeams+0x20] + 6 × id do clube: +0 vetor de entradas (data de 8 bytes, +8 convocados), +4 capacidade, +5 quantas; ponteiro e valor; lê e grava (registrar_convocados_por_clube)',
           '[NationalTeams+0x20] + 6 x club id: +0 entry array (8-byte date, +8 call-ups), +4 capacity, +5 count; pointer and value; reads and writes (registrar_convocados_por_clube)'),
          ('nação +0x11c: a nação marcada; valor; lê (registrar_convocados_por_clube)',
           'nation +0x11c: the nation flag; value; reads (registrar_convocados_por_clube)'),
          ('partida +0x28 / +0x2a: o ano e o dia; valor; lê (registrar_convocados_por_clube); +0x42: a fase; valor; lê (importancia_do_jogo_na_competicao)',
           'fixture +0x28 / +0x2a: year and day; value; reads (registrar_convocados_por_clube); +0x42: the stage; value; reads (importancia_do_jogo_na_competicao)'),
          ('competição +0xc / +0x30: as fases e a última; ponteiro e valor; lê (importancia_do_jogo_na_competicao, fase_da_competicao)',
           'competition +0xc / +0x30: the stages and the last one; pointer and value; reads (importancia_do_jogo_na_competicao, fase_da_competicao)')]
INTERESSA = ['CMRTE']
