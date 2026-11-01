# A Liga dos Campeões (european_cup.cpp, o que restava fora da fila): EuropeanCup::update_staff_records_for_status
# (0x584ae0, a virtual +0x44 da copa, chamada pelo anúncio do desfecho de cada clube: a qualificação de julho e agosto,
# as fases de grupos de setembro a março e o mata-mata até a final de maio de 2002, com os clubes de todas as nações),
# EuropeanCup::draw_quarter_finals (0x586330, da fase seguinte 0x5851d0 na fase 12, em março) e
# EuropeanCup::collect_club_list (0x586660, a virtual +0x58: a lista dos inscritos que seguem). Sem SEH na primeira e na
# última; a 0x586330 tem quadro SEH só no construtor da fase (sem destrutor). Iguais no cm0102-1080.exe.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x4bf850 (comp_util_scale_by_round): push 2; push rodada; push alto; push baixo; call; add esp, 0x10 -> cdecl de 4.
# - 0x86bdd0: 5 pushes (peso, rodada, fase, comp, clube); ecx = [0xdd7f00]; sem add esp -> thiscall, ret 0x14.
# - 0x86c1d0 (add_failed_to_qualify_record): push 0x32; push fase; push comp; push clube; ecx = [0xdd7f00] ->
#   thiscall(reg, clube, comp, fase, _), ret 0x10.
# - 0x86c000 (add_knocked_out_record): push peso; push rodada; push fase; push comp; push clube; ecx = [0xdd7f00] ->
#   thiscall(reg, clube, comp, fase, rodada, _), ret 0x14.
# - 0x86a800 / 0x86b0b0 (add_cup_winners_record / add_cup_runners_up_record): push final; push jogo; push clube; ecx =
#   [0xdd7f00] -> thiscall(reg, clube, jogo, final), ret 0xc.
# - 0x945501 (calloc): push 4; push 8; call; add esp, 8 -> cdecl(8, 4).
# - 0x90cfc0 (sorteio): push 4; call; add esp, 4 -> cdecl(n).
# - 0x6826d0 (League::get_table_row): push k; ecx = o grupo; sem add esp -> thiscall(grupo, k), ret 4.
# - 0x7e9180 (RulingBody::reset_bookings_for_club): push clube; ecx = [[[0xae28f0]] + 4 × +0x50]; sem add esp ->
#   thiscall(órgão, clube), ret 4.
# - virtual +0x3c: push 0; push &extra; push &rodadas; push 0xc; ecx = this -> thiscall, ret 0x10.
# - 0x944cf1 (operator new) cdecl(n); 0x522e00 (Cup_Stage::Cup_Stage) thiscall com 15 argumentos, ret 0x3c; 0x9452ca
#   (free) cdecl(p); 0x51c800 (Cup::set_up_round): push 0; ecx = a fase -> thiscall(fase, 0), ret 4.
# - 0x944e46 (malloc) cdecl(n); 0x48e1c0 (Comp::collect_club_list): push 0; push &lista; ecx = this; sem add esp ->
#   thiscall(this, &lista, 0), ret 8.
# Comparadores: a 0x584ae0 só a original (os registros vão ao arquivo e sorteiam o moral); o plano confere o +0x1db
# do clube (−1 quando ele sai, intacto nos outros casos). A 0x586330 só a original (sorteia e aloca); o plano: os 8
# primeiros e segundos dos grupos na fase nova (como conjunto), o 3º e o 4º de cada grupo fora, +0x30 = 12. A 0x586660
# aloca a lista (só a original); o plano, feito antes numa lista local, dá a conta e os mesmos clubes.
SAVE = False
ROTINAS = [('registros_da_liga_dos_campeoes', 0x584ae0, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('sorteio_das_quartas', 0x586330, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('clubes_da_liga_dos_campeoes', 0x586660, [0x8a, 0x44, 0x24, 0x08, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a Liga dos Campeões: os registros pela fase alcançada, o sorteio das quartas e a lista dos clubes inscritos',
        'the Champions League: the records by stage reached, the quarter-final draw and the entered club list',
        [('registros_da_liga_dos_campeoes', 'EuropeanCup::update_staff_records_for_status', '0x584ae0', 'european_cup.cpp', 'thiscall, ret 0x18',
          'pelo desfecho e pela fase: na qualificação, a classificação (0x86bdd0) ou a eliminação (0x86c000) com o peso pela rodada (0x4bf850); nas fases de grupos, a classificação (6) ou a não classificação (0xfe, 0x86c1d0); no mata-mata, o título (0x86a800), o vice (0x86b0b0), a classificação ou a eliminação; quem sai fica com +0x1db = −1; devolve 0',
          'by outcome and stage: in qualifying, the qualified (0x86bdd0) or knocked-out (0x86c000) record weighted by round (0x4bf850); in the group stages, qualified (6) or failed to qualify (0xfe, 0x86c1d0); in the knockout, winners (0x86a800), runners-up (0x86b0b0), qualified or knocked out; whoever leaves gets +0x1db = -1; returns 0'),
         ('sorteio_das_quartas', 'EuropeanCup::draw_quarter_finals', '0x586330', 'european_cup.cpp', 'thiscall, ret 0',
          'sorteia a ordem dos 4 grupos da fase 2 (0x90cfc0) e monta as quartas (1º de a × 2º de b, 2º de a × 1º de b, idem c e d), zera os cartões dos 8 (0x7e9180), cria a fase do mata-mata (0x522e00) na vaga +0x30, tira o 3º e o 4º de cada grupo (+0x1db = −1), monta a rodada (0x51c800) e +0x30 = 12',
          'draws the order of the 4 phase 2 groups (0x90cfc0) and pairs the quarter-finals (1st of a v 2nd of b, 2nd of a v 1st of b, same for c and d), resets the 8 clubs bookings (0x7e9180), creates the knockout stage (0x522e00) in slot +0x30, drops the 3rd and 4th of each group (+0x1db = -1), sets up the round (0x51c800) and +0x30 = 12'),
         ('clubes_da_liga_dos_campeoes', 'EuropeanCup::collect_club_list', '0x586660', 'european_cup.cpp', 'thiscall, ret 8',
          'aloca a lista (0x11c bytes) dos inscritos (+0x14) que seguem na copa (+0x1db = o id dela): tipo 1 sem os da rodada 7, tipo 2 todos, tipo 3 na fase 11; noutro caso a da competição (0x48e1c0); devolve a conta',
          'allocates the list (0x11c bytes) of the entrants (+0x14) still in the cup (+0x1db = its id): kind 1 without the round 7 entries, kind 2 all, kind 3 at stage 11; otherwise the competition one (0x48e1c0); returns the count')])
CAMPOS = [('clube +0x1db: a competição europeia da temporada; id; lê (clubes_da_liga_dos_campeoes), grava −1 (registros_da_liga_dos_campeoes, sorteio_das_quartas)',
           "club +0x1db: the season's European competition; id; reads (clubes_da_liga_dos_campeoes), writes -1 (registros_da_liga_dos_campeoes, sorteio_das_quartas)"),
          ('jogo +0x30 / +0x32 / +0x34: as palavras da rodada que vão aos registros; valor; lê (registros_da_liga_dos_campeoes)',
           'fixture +0x30 / +0x32 / +0x34: the round words passed to the records; value; reads (registros_da_liga_dos_campeoes)'),
          ('Liga dos Campeões +0xc / +0x30: as fases (+0x20..+0x2c os grupos da fase 2, +0x30 o mata-mata) e a fase atual; ponteiro e valor; lê, grava a vaga +0x30 e a fase 12 (sorteio_das_quartas)',
           'Champions League +0xc / +0x30: the stages (+0x20..+0x2c the phase 2 groups, +0x30 the knockout) and the current stage; pointer and value; reads, writes slot +0x30 and stage 12 (sorteio_das_quartas)'),
          ('Liga dos Campeões +0x14 / +0x38: a lista dos inscritos (6 bytes: +0 clube, +4 rodada de entrada) e a conta; ponteiro e valor; lê (clubes_da_liga_dos_campeoes)',
           'Champions League +0x14 / +0x38: the entrant list (6 bytes: +0 club, +4 entry round) and the count; pointer and value; reads (clubes_da_liga_dos_campeoes)'),
          ('Liga dos Campeões +0x3a / +0xa3 / +0x50: a fase da qualificação, a tabela de rodadas e o índice do órgão; valor e ponteiro; lê (registros_da_liga_dos_campeoes, sorteio_das_quartas)',
           'Champions League +0x3a / +0xa3 / +0x50: the qualifying stage, the round table and the ruling body index; value and pointer; reads (registros_da_liga_dos_campeoes, sorteio_das_quartas)')]
INTERESSA = ['CMRTE']
