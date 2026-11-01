# Os inscritos da Liga dos Campeões (european_cup.cpp): EuropeanCup::collect_entrants (0x584fa0: os clubes cuja
# competição europeia (+0x1db) é a Liga dos Campeões, pela banda (+0x1df), nas quatro faixas da lista de 71; do
# construtor 0x5837c0 na carreira nova, 0x583888, e do começo da temporada 0x584e5d),
# EuropeanCup::build_qualifying_entry_table (0x584990: as 55 linhas da qualificação, de trás para a frente, com a
# rodada de entrada; logo depois, 0x58388f/0x584e9a) e EuropeanCup::advance_stage (0x5851d0, a virtual +0x28 do
# passe da copa 0x51f904: a fase seguinte e os sorteios da fase 1, da fase 2 e das quartas). Iguais no
# cm0102-1080.exe; sem SEH; sem x87. As duas primeiras não estão na cobertura de 17/10 (european_cup.cpp não foi
# instrumentado), mas o construtor roda em toda carreira nova (a Liga dos Campeões é montada com os clubes de todas
# as nações, ligas carregadas ou não).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl(n).
# - 0x9452ca (free): push p; call; add esp, 4 -> cdecl(p).
# - 0x585290 (sorteio da fase 1): mov ecx = this (o da entrada), sem push, sem add esp -> thiscall(this), ret 0.
# - 0x585ab0 (sorteio da fase 2): push 0; ecx = this; sem add esp -> thiscall(this, 0), ret 4 (0x58632c).
# - 0x586330 (as quartas): ecx = this, sem push -> thiscall(this), ret 0 (0x586659).
# - a caixa de erro (aviso_em do motor.rs: 0x944cff cdecl 5, 0x90d130 cdecl 3, 0x944c9f cdecl, 0x5e8290 cdecl 3),
#   com o fonte 0x9c4900 (european_cup.cpp), como nas outras rotinas.
# Comparadores: a 0x584fa0 e a 0x584990 alocam (só a original roda) e o plano confere tudo o que gravaram: a lista
# (ou a tabela) nova copiada e refeita pela cópia sai igual, com +0x36/+0x38 (ou +0x3e) e a devolução. A 0x5851d0 só
# a original (os sorteios sorteiam e alocam); a cópia leva o ramo.
SAVE = False
ROTINAS = [('inscritos_da_liga_dos_campeoes', 0x584fa0, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('tabela_da_qualificacao_da_liga_dos_campeoes', 0x584990, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('proxima_fase_da_liga_dos_campeoes', 0x5851d0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('os inscritos da Liga dos Campeões: a lista das quatro bandas, a tabela da qualificação e a fase seguinte',
        'the Champions League entrants: the four-band entry list, the qualifying table and the next stage',
        [('inscritos_da_liga_dos_campeoes', 'EuropeanCup::collect_entrants', '0x584fa0', 'european_cup.cpp', 'thiscall, ret 0',
          'aloca a lista de 71 entradas de 6 bytes (+0x14) e põe nela os clubes da Liga dos Campeões (+0x1db = [0x9cf6e8]) pela banda (+0x1df): 1 até 16 (rodada 7), 2 até 18 (8), 3 até 19 (0xd), 4 até 18 (0); sem as 71, +0x36 = 0 e devolve 0',
          'allocates the 71-entry list of 6-byte entries (+0x14) and fills it with the Champions League clubs (+0x1db = [0x9cf6e8]) by band (+0x1df): 1 up to 16 (round 7), 2 up to 18 (8), 3 up to 19 (0xd), 4 up to 18 (0); short of 71, +0x36 = 0 and returns 0'),
         ('tabela_da_qualificacao_da_liga_dos_campeoes', 'EuropeanCup::build_qualifying_entry_table', '0x584990', 'european_cup.cpp', 'thiscall, ret 0',
          'aloca as 55 linhas da qualificação (+0xa7, +0x3e = 55): a linha i leva o clube da entrada 70 − i e a rodada em que entra (8 → 2, 0xd → 1, outra → 0); sem memória, −1',
          'allocates the 55 qualifying rows (+0xa7, +0x3e = 55): row i takes the club of entry 70 - i and its entry round (8 -> 2, 0xd -> 1, other -> 0); out of memory, -1'),
         ('proxima_fase_da_liga_dos_campeoes', 'EuropeanCup::advance_stage', '0x5851d0', 'european_cup.cpp', 'thiscall, ret 0',
          'passa à fase seguinte (+0x30) e sorteia os grupos da fase 1 (fase 0), os da fase 2 (8) ou as quartas (12); na última fase, a caixa de erro',
          'moves to the next stage (+0x30) and draws the phase 1 groups (stage 0), the phase 2 groups (8) or the quarter-finals (12); at the last stage, the error box')])
CAMPOS = [('clube +0x1db: a competição europeia da temporada; id; lê (inscritos_da_liga_dos_campeoes)',
           "club +0x1db: the season's European competition; id; reads (inscritos_da_liga_dos_campeoes)"),
          ('clube +0x1df: a banda de entrada na Liga dos Campeões (1 a 4); valor; lê (inscritos_da_liga_dos_campeoes)',
           'club +0x1df: the Champions League entry band (1 to 4); value; reads (inscritos_da_liga_dos_campeoes)'),
          ('[0x9cf6e8] a Liga dos Campeões; id; lê (inscritos_da_liga_dos_campeoes)',
           '[0x9cf6e8] the Champions League; id; reads (inscritos_da_liga_dos_campeoes)'),
          ('Liga dos Campeões +0x14 / +0x36 / +0x38: a lista dos inscritos (6 bytes: +0 clube, +4 rodada, +5 0) e as contas; ponteiro e valor; grava (inscritos_da_liga_dos_campeoes), lê (tabela_da_qualificacao_da_liga_dos_campeoes)',
           'Champions League +0x14 / +0x36 / +0x38: the entrant list (6 bytes: +0 club, +4 round, +5 0) and the counts; pointer and value; writes (inscritos_da_liga_dos_campeoes), reads (tabela_da_qualificacao_da_liga_dos_campeoes)'),
          ('Liga dos Campeões +0xa7 / +0x3e: a tabela da qualificação (6 bytes: +0 clube, +4 rodada de entrada 0/1/2, +5 0) e quantas linhas; ponteiro e valor; grava (tabela_da_qualificacao_da_liga_dos_campeoes)',
           'Champions League +0xa7 / +0x3e: the qualifying table (6 bytes: +0 club, +4 entry round 0/1/2, +5 0) and the row count; pointer and value; writes (tabela_da_qualificacao_da_liga_dos_campeoes)'),
          ('Liga dos Campeões +0x30 / +0x2c: a fase atual e quantas fases; valor; lê e grava a fase (proxima_fase_da_liga_dos_campeoes)',
           'Champions League +0x30 / +0x2c: the current stage and the stage count; value; reads and writes the stage (proxima_fase_da_liga_dos_campeoes)')]
INTERESSA = ['CMRTE']
