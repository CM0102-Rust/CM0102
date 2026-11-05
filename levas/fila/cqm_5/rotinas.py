# Os construtores da segunda e da terceira divisões francesas e da Primera e da Segunda espanholas (Agente B da frente
# das competições, 04/11/2026). Rodam uma vez cada na criação da carreira de 8 ligas (docs/cobertura_competicoes.txt):
# - 0x5be330 (FraSecond::FraSecond, 526 bytes), 0x5bfb40 (FraThird::FraThird, 526), 0x8539a0 (SpaSecond::SpaSecond,
#   526) e 0x84e130 (SpaFirst::SpaFirst, 606), com quadro SEH só para os operator new.
# Iguais no cm0102-1080.exe (inteiros). Diferem na vtable, na disciplina (0xc a francesa, 0x1a a espanhola), nas regras,
# na tabela, no fonte e nas linhas das caixas; a Primera cria no fim a divisão de baixo espanhola (conferido pela
# desmontagem).
# Convenções conferidas nos pontos de chamada da original:
# - 0x682200 (League::League): mov ecx, esi -> thiscall, ret 0;
# - 0x687b10 (League::load_from_temp_file): push 1; mov ecx, esi -> thiscall, ret 4;
# - as regras (0x5bf0a0, 0x5c0700, 0x855cc0, 0x84fdf0) e as tabelas (0x5bef10, 0x5c05d0, 0x855b40, 0x84fc70): mov ecx,
#   esi -> thiscall, ret 0;
# - 0x6835c0 (League::create_fixtures): mov ecx, esi -> thiscall, ret 0;
# - 0x6827d0 (League::sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x944cf1 (operator new): push 0x5ce / push 0xee; add esp, 4 -> cdecl;
# - 0x49ee70 (Comp_Stats::Comp_Stats): push esi; mov ecx, novo -> thiscall, ret 4;
# - 0x68a850 (League::set_predicted_positions): mov ecx, esi -> thiscall, ret 0;
# - 0x851020 (SpaLower::SpaLower): push ecx (cx = o ano, os bits de cima são do ponteiro do registro; o construtor só lê
#   a palavra); mov ecx, novo -> thiscall, ret 4;
# - a caixa: aviso_em.
# Comparador: só a original (aloca), conferindo a devolução, a vtable, o ano, o registro, a disciplina, as fases e, na
# Primera, a divisão de baixo em +0xee conforme o bit 4 da nação.
# Determinismo: só constantes, os argumentos e o bit da nação.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construir_franca_2', 0x5be330, [0x6a, 0xff, 0x68, 0x89, 0x9a, 0x95, 0x00]),
           ('construir_franca_3', 0x5bfb40, [0x6a, 0xff, 0x68, 0x29, 0x9b, 0x95, 0x00]),
           ('construir_segunda', 0x8539a0, [0x6a, 0xff, 0x68, 0x59, 0x27, 0x96, 0x00]),
           ('construir_primera', 0x84e130, [0x6a, 0xff, 0x68, 0xa7, 0x26, 0x96, 0x00])]
DOCS = ('os construtores da segunda e da terceira divisões francesas e da Primera e da Segunda espanholas',
 'the constructors of the French second and third divisions and the Spanish Primera and Segunda',
 [('construir_franca_2', 'FraSecond::FraSecond', '0x5be330', 'fra_second.cpp', 'thiscall, ret 8',
   'a segunda divisão francesa: a regra de disciplina 0xc (a francesa), só a liga; sem estado salvo, as regras, a '
   'tabela, os jogos, a ordem, as estatísticas e as posições previstas',
   'the French second division: discipline rule 0xc (the French one), only the league; with no saved state, the '
   'rules, the table, the fixtures, the order, the stats and the predicted positions'),
  ('construir_franca_3', 'FraThird::FraThird', '0x5bfb40', 'fra_third.cpp', 'thiscall, ret 8',
   'a terceira divisão francesa (o National): disciplina 0xc, só a liga',
   'the French third division (the National): discipline 0xc, only the league'),
  ('construir_segunda', 'SpaSecond::SpaSecond', '0x8539a0', 'spa_second.cpp', 'thiscall, ret 8',
   'a Segunda espanhola: a regra de disciplina 0x1a (a espanhola), só a liga',
   'the Spanish Segunda: discipline rule 0x1a (the Spanish one), only the league'),
  ('construir_primera', 'SpaFirst::SpaFirst', '0x84e130', 'spa_first.cpp', 'thiscall, ret 8',
   'a Primera: disciplina 0x1a, só a liga; por fim, se a nação não tem o bit 4 de +0x11c [inferência: a divisão de baixo '
   'jogável], cria a divisão de baixo espanhola (os times B e os melhores da Segunda B) em +0xee',
   'the Primera: discipline 0x1a, only the league; finally, if the nation lacks bit 4 of +0x11c [inference: the lower '
   'division playable], it creates the Spanish lower division (the B teams and the best of Segunda B) at +0xee')])
CAMPOS = [('liga +0 vtable, +4 registro da competição, +0x40 ano, +0x50 regra de disciplina; valor e ponteiro; grava (os '
           'quatro)',
           'league +0 vtable, +4 competition record, +0x40 year, +0x50 discipline rule; value and pointer; writes'),
          ('liga +0x44 fase, +0x30 fase atual, +0x2c fases, +0xc vetor das fases, +8 Comp_Stats; valor e ponteiro; grava',
           'league +0x44 stage, +0x30 current stage, +0x2c stages, +0xc stage vector, +8 Comp_Stats; value and pointer; '
           'writes'),
          ('Primera +0xee a divisão de baixo espanhola; ponteiro; grava (construir_primera)',
           'Primera +0xee the Spanish lower division; pointer; writes'),
          ('nação +0x11c bit 4 [inferência: a divisão de baixo jogável]; valor; lê (construir_primera)',
           'nation +0x11c bit 4 [inference: lower division playable]; value; reads')]
INTERESSA = ['CMRTE']
