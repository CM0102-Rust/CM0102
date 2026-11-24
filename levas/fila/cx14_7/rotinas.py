# O mata-mata da segunda divisao turca categoria B e a divisao dos clubes dela nos cinco grupos regionais na virada
# (tur_second_b.cpp, tur_first.cpp; leitor cx14, rodada 5, 24/11/2026). Simulacao com a Turquia (as ligas e a data dos
# itens mwd): o mata-mata no fim dos grupos de classificacao (primavera de 2002, 1 chamada na cobertura do mundo, pelo
# advance_stage do mwd_31), os grupos regionais na virada do verao de 2002 (pelo tur_first_start_new_season do mwd_24).
# - 0x901040 TurSecondB::create_playoff_stage thiscall() ret 0: as linhas de status 1 da fase 4 num local de 8 (caixa
#   0x37e), o primeiro de cada fase 5..9 (caixa 0x389); a tabela de rodadas da fase 10 (virtual +0x3c (10, &rodadas,
#   &extra, 0)); Cup_Stage 0x522e00 (operator new 0xb2; thiscall, ret 0x3c) com 8, a lista, as rodadas, a competicao,
#   a tabela, o ano, 10, 1, o extra, 0x14, 0, 0, 0, 0 (pushes conferidos); sem a fase, caixa 0x395; senao free da
#   tabela, Cup::set_up_round 0x51c800 (thiscall(0)) e fase atual = 10.
# - 0x8fe580 tur_first_assign_third_tier_groups thiscall() ret 0 (ecx nao lido): os clubes da categoria B (divisao com
#   a id da chave 0x9cf8e0) num local de ate 50, qsort 0x945333 com 0x53fe50 (cdecl) e a divisao reserva (+0x60) dos
#   grupos 1..5 (registros [0xae23d0] + id x 0x6b das chaves 0x9cf8e4..0x9cf8f4, lidos na entrada) de dez em dez.
# Prologos: mov eax, fs:[0] (6 bytes); sub esp, 0xd4 (6 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: o
# mata-mata so a original (aloca e sorteia), os 8 clubes contra o plano (como conjunto) e a fase atual; os grupos
# regionais lado a lado (a divisao reserva dos clubes da categoria B).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('tur_second_b_create_playoff_stage', 0x901040, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('tur_first_assign_third_tier_groups', 0x8fe580, [0x81, 0xec, 0xd4, 0x00, 0x00, 0x00])]
DOCS = ('o mata-mata da categoria B turca e os seus grupos regionais',
        'the Turkish category B play-off and its regional groups',
        [('tur_second_b_create_playoff_stage', 'TurSecondB::create_playoff_stage', '0x901040', 'tur_second_b.cpp', 'thiscall, ret 0',
          'regra: o mata-mata da categoria B: os três do grupo de acesso que vão ao mata-mata e o primeiro de cada um dos cinco grupos de classificação, oito clubes em copa',
          'rule: the category B play-off: the three of the promotion group that go to the play-off and the first of each of the five classification groups, eight clubs in a cup'),
         ('tur_first_assign_third_tier_groups', 'tur_first_assign_third_tier_groups', '0x8fe580', 'tur_first.cpp', 'thiscall, ret 0',
          'regra: na virada, os (até 50) clubes da categoria B são ordenados pela posição das cidades e repartidos de dez em dez nos grupos 1 a 5',
          'rule: at the turnover, the (up to 50) category B clubs are sorted by where their cities are and dealt ten at a time into groups 1 to 5')])
CAMPOS = []
INTERESSA = []
