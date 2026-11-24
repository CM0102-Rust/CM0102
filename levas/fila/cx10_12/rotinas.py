# Os codigos de progresso e as vagas fixas do sorteio da Divisao Principal irlandesa (ire_prm.cpp; leitor cx10, rodada
# 5, 24/11/2026). Competicao de outra nacao: o progresso a cada rodada de liga; as vagas fixas sao virtuais (sem chamada
# direta), lidas e gravadas quando o calendario da liga e sorteado (criacao do jogo e virada de temporada).
# - 0x63ab60 IrePrm::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (slot SET_CLUB_PROGRESS):
#   Comp_Stats::find_club_entry 0x4a4850 (mov ecx, [this+8]; push clube -> thiscall, ret 4); o pior 10 vira 11; o
#   melhor e comparado com 10 e o salto cai na instrucao seguinte (nada muda: defeito do original).
# - 0x63aba0 IrePrm::get_fixed_draw_slot thiscall(clube) ret 4 -> byte: as 10 entradas de 5 bytes em +0xee (id do clube,
#   a vaga em +4); sem a entrada, 0xff (o resto do eax e 0).
# - 0x63abe0 IrePrm::set_fixed_draw_slot thiscall(clube, vaga) ret 8: na entrada do clube; senao na primeira livre (-1) ou
#   de clube cuja divisao (clube +0x57, a tabela [0xae23b4], 0x245 bytes) nao e esta competicao (+4); nenhuma: nada.
# Prologos mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes) e mov eax, [esp+4]; push esi/ebx (5 bytes). Iguais no
# cm0102-1080.exe (inteiras).
# Comparadores: o progresso pelo compare_club_progress do cx02_4; a leitura das vagas lado a lado (o byte devolvido); a
# gravacao lado a lado sobre os 50 bytes da tabela.
# Depende do cx02_4 (compare_club_progress) e do cx03_9 (store_club_progress): integrar depois deles.
SAVE = False
LIGAS = 'ENG IRL NIR'
ROTINAS = [('ire_prm_set_club_progress', 0x63ab60, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('ire_prm_get_fixed_draw_slot', 0x63aba0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('ire_prm_set_fixed_draw_slot', 0x63abe0, [0x8b, 0x44, 0x24, 0x04, 0x53])]
DOCS = ('os códigos de progresso e as vagas fixas do sorteio da Divisão Principal irlandesa',
        'the Irish Premier Division\'s progress codes and fixed draw slots',
        [('ire_prm_set_club_progress', 'IrePrm::set_club_progress', '0x63ab60', 'ire_prm.cpp', 'thiscall, ret 0x14',
          'guarda os três códigos de progresso do clube nas estatísticas; o pior código 10 vira 11 (o melhor é comparado com 10 e não muda)',
          'stores the club\'s three progress codes in the statistics; the worst code 10 becomes 11 (the best is compared with 10 and does not change)'),
         ('ire_prm_get_fixed_draw_slot', 'IrePrm::get_fixed_draw_slot', '0x63aba0', 'ire_prm.cpp', 'thiscall, ret 4',
          'devolve a vaga fixa do sorteio guardada para o clube numa das 10 entradas, ou −1 quando ele não tem',
          'returns the fixed draw slot stored for the club in one of the 10 entries, or −1 when it has none'),
         ('ire_prm_set_fixed_draw_slot', 'IrePrm::set_fixed_draw_slot', '0x63abe0', 'ire_prm.cpp', 'thiscall, ret 8',
          'guarda a vaga fixa do clube: na entrada dele, ou na primeira livre ou de um clube de outra divisão; sem nenhuma, nada',
          'stores the club\'s fixed draw slot: in its own entry, or in the first one free or held by a club of another division; with none, nothing')])
CAMPOS = []
INTERESSA = []
