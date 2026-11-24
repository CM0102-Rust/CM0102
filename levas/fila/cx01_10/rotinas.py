# A Copa dos Campeoes do Brasil: o construtor, os inscritos e as posicoes previstas (bra_champ_cup.cpp; leitor cx01,
# rodada 5, 24/11/2026). Competicoes com o Brasil (carreira de 8 ligas: o construtor roda 1 vez na criacao, pela
# setup_set_game_start_date 0x833046; a copa joga a preliminar em junho de 2001).
# - 0x4259a0 BraChampCup::BraChampCup thiscall(ano, registro) ret 8 (SEH so para o new das estatisticas): League::League
#   0x682200 (mov ecx -> thiscall, ret 0); League::load_from_temp_file 0x687b10 (push 1 -> thiscall, ret 4); malloc 0xc
#   cdecl; 0x426d80 (call direto com o ecx da copa); operator new 0x5ce e Comp_Stats::Comp_Stats 0x49ee70 (push copa;
#   mov ecx, novo -> thiscall, ret 4); BraChampCup::set_up_rules 0x426230 (thiscall, ret 0; ja Rust); as caixas das
#   linhas 0x53 e 0x5a.
# - 0x426d80 BraChampCup::select_entrants thiscall() ret 0: qsort cdecl(lista, n, 4, 0x4b07d0); calloc cdecl(6, 9) (os
#   dois push 9 e 6: 54 bytes); a caixa 0x3d7; os nomes comparados com strcmp em linha.
# - 0x426ab0 BraChampCup::set_predicted_positions thiscall() ret 0 (slot 0x5c): malloc 0x24; Comp_Stats::
#   set_club_final_position 0x4a2540 (push posicao; push clube; mov ecx, [+8] -> thiscall, ret 8); free; as caixas 0x319
#   e 0x321.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: o construtor so a original, conferindo vtable, ano, registro, banco, disciplina e importancia; os
# inscritos lado a lado (a copia monta outro livro, comparado e liberado); as posicoes previstas so a original (pode
# acrescentar clubes as estatisticas), conferindo as nove contra a tabela da copia.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construct_bra_champ_cup', 0x4259a0, [0x6a, 0xff, 0x68, 0x39, 0x43, 0x95, 0x00]),
           ('bra_champ_cup_select_entrants', 0x426d80, [0x81, 0xec, 0x2c, 0x05, 0x00, 0x00]),
           ('bra_champ_cup_set_predicted_positions', 0x426ab0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa dos Campeões do Brasil: o construtor, os inscritos e as posições previstas',
        'the Brazilian Champions Cup: constructor, entrants and predicted positions',
        [('construct_bra_champ_cup', 'BraChampCup::BraChampCup', '0x4259a0', 'bra_champ_cup.cpp', 'thiscall, ret 8',
          'monta a Copa dos Campeões sobre uma liga: 3 substituições, 7 reservas, disciplina 6, importância 12; com estado salvo, só isso; senão três fases, os inscritos, as estatísticas e as regras, e a copa fica ainda sem tabela nem clubes',
          'builds the Champions Cup on a league: 3 substitutions, 7 substitutes named, discipline 6, importance 12; with a saved state, that is all; otherwise three stages, the entrants, the statistics and the rules, and the cup is left without a table or clubs yet'),
         ('bra_champ_cup_select_entrants', 'BraChampCup::select_entrants', '0x426d80', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'escolhe os nove inscritos: os clubes nomeados (Vasco ou Vila Nova, Náutico, São Raimundo, América-MG, Grêmio, Flamengo, Atlético-PR, Corinthians, Bahia) em suas vagas; cada vaga vazia fica com o clube de maior reputação do estadual dela',
          'picks the nine entrants: the named clubs (Vasco or Vila Nova, Náutico, São Raimundo, América-MG, Grêmio, Flamengo, Atlético-PR, Corinthians, Bahia) in their places; each empty place goes to the highest-reputation club of its state championship'),
         ('bra_champ_cup_set_predicted_positions', 'BraChampCup::set_predicted_positions', '0x426ab0', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'grava as posições previstas fixas dos nove inscritos, na ordem: 1, 2, 3, 3, 5, 5, 7, 7 e 9',
          'stores the fixed predicted positions of the nine entrants, in order: 1, 2, 3, 3, 5, 5, 7, 7 and 9')])
CAMPOS = []
INTERESSA = []
