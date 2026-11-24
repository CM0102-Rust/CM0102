# Os grupos e as semifinais da Copa das Confederacoes (fifa_confed.cpp; leitor cx06, rodada 5, 24/11/2026). Simulacao
# com as 8 ligas e ATE 2003-08-15 (a copa de 2003: o grupo A e a fase do grupo B no dia do sorteio pelo plano
# FIFA_CONFED_DAY ja em Rust, as semifinais em 06/08/2003 pelo advance_stage do cx06_8); 0 chamadas na cobertura do mundo
# ate 2002-08-01: sem a data longa o item nao prova nada (o mesmo ATE do cx04_6).
# - 0x58bcb0 FifaConfed::create_group_table thiscall() ret 0 -> 0/1: +0x3e = 4, +0xb1 = malloc(0xec) (nulo: caixa 0x1c0
#   e 0), qsort 0x945333 da lista +0xee pela reputacao (0x4b07d0), as linhas 0..3 com as selecoes 0, 2, 4, 6
#   (League::init_table_row 0x687430: push 0; push 0; push selecao; push linha -> thiscall, ret 0x10; a linha num dword
#   com lixo acima do byte, que a rotina nao le), League::send_group_draw_news 0x684230 e 1.
# - 0x58c100 FifaConfed::create_group_stage thiscall() ret 0 -> 0/1: malloc(0x10) (nulo: caixa 0x2b3 e 0) com as selecoes
#   1, 3, 5, 7; a tabela de rodadas da fase 0 (virtual 0x3c: 0, &rodadas, &id, 0); operator new 0xee e League_Stage
#   0x68b100 (a copa, 4, a lista, 1, +4, a tabela, as rodadas, +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano, 0, o id, +0x51,
#   2, 0, +0xd9, -1, 0, 2) em [+0xc][0]; a lista e a tabela liberadas; nula: caixa 0x2ca e 0; +0x30 = 0, a noticia do
#   sorteio na fase e 1.
# - 0x58c2d0 FifaConfed::create_knockout_stage thiscall() ret 0: malloc(0x10) (nulo: caixa 0x2e8) com o 2o do grupo A, o
#   1o do B, o 1o do A e o 2o do B; a tabela de rodadas da fase 1; operator new 0xb2 e Cup_Stage 0x522e00 (thiscall,
#   ret 0x3c: a copa, 4, a lista, as rodadas, +4, a tabela, o ano, 1, 2, o id, 0x14, 0, 0, 0, 0) em [+0xc][1]; a tabela e
#   a lista liberadas; nula: caixa 0x300; Cup::set_up_round 0x51c800 (push 0) e +0x30 = 1. Prologo mov eax, fs:[0] (6
#   bytes) nesta e na anterior.
# Os anos, as rodadas e o id vao em registros com lixo nos 16 bits altos, que os construtores nao leem: a copia passa os
# valores limpos. Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: so a original (alocam e mandam noticias), com o plano (as linhas do grupo A; os times da fase do grupo
# B, como conjunto, e +0x30; os 4 cruzados na lista da copa, como conjunto, e +0x30).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('fifa_confed_create_group_table', 0x58bcb0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('fifa_confed_create_group_stage', 0x58c100, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('fifa_confed_create_knockout_stage', 0x58c2d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os grupos e as semifinais da Copa das Confederações', "the Confederations Cup's groups and semi-finals",
        [('fifa_confed_create_group_table', 'FifaConfed::create_group_table', '0x58bcb0', 'fifa_confed.cpp', 'thiscall, ret 0',
          'monta o grupo A na própria copa: as 8 seleções em ordem de reputação, e o grupo A fica com a 1ª, a 3ª, a 5ª e a 7ª; manda a notícia do sorteio',
          'builds group A in the cup itself: the 8 nations by reputation, and group A takes the 1st, 3rd, 5th and 7th; sends the draw news'),
         ('fifa_confed_create_group_stage', 'FifaConfed::create_group_stage', '0x58c100', 'fifa_confed.cpp', 'thiscall, ret 0',
          'cria o grupo B como a fase 0, com a 2ª, a 4ª, a 6ª e a 8ª seleções em reputação; manda a notícia do sorteio',
          'creates group B as stage 0, with the 2nd, 4th, 6th and 8th nations by reputation; sends the draw news'),
         ('fifa_confed_create_knockout_stage', 'FifaConfed::create_knockout_stage', '0x58c2d0', 'fifa_confed.cpp', 'thiscall, ret 0',
          'cria as semifinais como a fase 1, cruzando os grupos: 2º do A contra 1º do B e 1º do A contra 2º do B',
          'creates the semi-finals as stage 1, crossing the groups: A runner-up against B winner and A winner against B runner-up')])
CAMPOS = []
INTERESSA = []
