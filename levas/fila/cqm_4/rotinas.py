# Os construtores da Série A, da Série B e dos dois grupos da Série C1 italianas (Agente B da frente das competições,
# 04/11/2026). Rodam uma vez cada na criação da carreira de 8 ligas (docs/cobertura_competicoes.txt):
# - 0x6406d0 (ItaSerA::ItaSerA, 546 bytes), 0x645810 (ItaSerB::ItaSerB, 546), 0x64aa70 (ItaSerC1a::ItaSerC1a, 547) e
#   0x64ec20 (ItaSerC1b::ItaSerC1b, 547), com quadro SEH só para o operator new das estatísticas.
# Iguais no cm0102-1080.exe (inteiros). Diferem na vtable, nas fases (a A e a B: 0 fases, +0xee = 0 e +0x4c = 1; a C1:
# 2 fases, o vetor [+0xc] = malloc de 8), nas regras, na tabela, no calendário real de 2001/02, no fonte e nas linhas
# das caixas (conferido pela desmontagem).
# Convenções conferidas nos pontos de chamada da original:
# - 0x682200 (League::League): mov ecx, esi -> thiscall, ret 0;
# - 0x687b10 (League::load_from_temp_file): push 1; mov ecx, esi -> thiscall, ret 4;
# - 0x944e46 (malloc): push 8; add esp, 4 -> cdecl (só a C1);
# - as regras (0x641640, 0x6468a0, 0x64b770, 0x64f920), as tabelas (0x641510, 0x646780, 0x64b670, 0x64f820) e o
#   calendário real (0x642a70, 0x647230, 0x64be90, 0x650100, este sem conferir a devolução): mov ecx, esi -> thiscall,
#   ret 0;
# - 0x6835c0 (League::create_fixtures): mov ecx, esi -> thiscall, ret 0;
# - 0x6827d0 (League::sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x944cf1 (operator new): push 0x5ce; add esp, 4 -> cdecl;
# - 0x49ee70 (Comp_Stats::Comp_Stats): push esi; mov ecx, novo -> thiscall, ret 4;
# - 0x68a850 (League::set_predicted_positions): mov ecx, esi -> thiscall, ret 0;
# - a caixa: aviso_em.
# Comparador: só a original (aloca), conferindo a devolução, a vtable, o ano, o registro, a disciplina e, na liga
# nova, as fases e +0xee.
# Determinismo: só constantes e os argumentos.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construir_serie_a', 0x6406d0, [0x6a, 0xff, 0x68, 0x59, 0xb9, 0x95, 0x00]),
           ('construir_serie_b', 0x645810, [0x6a, 0xff, 0x68, 0x09, 0xba, 0x95, 0x00]),
           ('construir_c1a', 0x64aa70, [0x6a, 0xff, 0x68, 0x99, 0xba, 0x95, 0x00]),
           ('construir_c1b', 0x64ec20, [0x6a, 0xff, 0x68, 0x39, 0xbb, 0x95, 0x00])]
DOCS = ('os construtores da Série A, da Série B e dos dois grupos da Série C1 italianas',
 'the constructors of the Italian Serie A, Serie B and the two Serie C1 groups',
 [('construir_serie_a', 'ItaSerA::ItaSerA', '0x6406d0', 'ita_ser_a.cpp', 'thiscall, ret 8',
   'a Série A: a regra de disciplina 0x11 (a das ligas italianas), só a liga (o desempate nasce no fim da temporada); '
   'sem estado salvo, as regras, a tabela, o calendário real de 2001/02 pelo nome dos clubes, os jogos, a ordem, as '
   'estatísticas e as posições previstas',
   'Serie A: discipline rule 0x11 (the Italian leagues one), only the league (the tie-break is born at season end); with '
   'no saved state, the rules, the table, the real 2001/02 fixture list by club name, the fixtures, the order, the '
   'stats and the predicted positions'),
  ('construir_serie_b', 'ItaSerB::ItaSerB', '0x645810', 'ita_ser_b.cpp', 'thiscall, ret 8',
   'a Série B: disciplina 0x11, só a liga, o calendário real de 2001/02',
   'Serie B: discipline 0x11, only the league, the real 2001/02 fixture list'),
  ('construir_c1a', 'ItaSerC1a::ItaSerC1a', '0x64aa70', 'ita_ser_c1a.cpp', 'thiscall, ret 8',
   'a C1/A: disciplina 0x11, a liga e 2 fases (o play-off de acesso e o play-out), o calendário real de 2001/02',
   'C1/A: discipline 0x11, the league and 2 stages (the promotion play-off and the play-out), the real 2001/02 '
   'fixture list'),
  ('construir_c1b', 'ItaSerC1b::ItaSerC1b', '0x64ec20', 'ita_ser_c1b.cpp', 'thiscall, ret 8',
   'a C1/B, como a C1/A', 'C1/B, as C1/A')])
CAMPOS = [('liga +0 vtable, +4 registro da competição, +0x40 ano, +0x50 regra de disciplina; valor e ponteiro; grava (os '
           'quatro)',
           'league +0 vtable, +4 competition record, +0x40 year, +0x50 discipline rule; value and pointer; writes'),
          ('liga +0x44 fase, +0x30 fase atual, +0x2c fases, +0xc vetor das fases, +8 Comp_Stats, +0x4c marca do '
           'histórico, +0xee (byte da classe na A e na B) [inferência: o desempate em curso]; valor e ponteiro; grava',
           'league +0x44 stage, +0x30 current stage, +0x2c stages, +0xc stage vector, +8 Comp_Stats, +0x4c history mark, '
           '+0xee (class byte in A and B) [inference: tie-break in progress]; value and pointer; writes')]
INTERESSA = ['CMRTE']
