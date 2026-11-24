# A tabela do ranking da UEFA para as telas e os voluntarios ingleses da Intertoto (uefa_seeding.cpp; leitor cx15,
# rodada 5, 24/11/2026). Item de TELAS: as duas so sao chamadas por telas (0x49c9db e 0x49cfa2, ecx = [0xde1f58]).
# Caminhos no jogo: a tela do ranking das nacoes da UEFA (os coeficientes) mostra get_nation_table; a noticia 0x16 do
# pedido de voluntarios da Intertoto (dia 100 do ano, para o tecnico humano de um clube da Premier) e a tela em que ele
# se oferece chamam add_intertoto_volunteer. Jogando comparando.
# - 0x903590 UefaSeeding::get_nation_table thiscall(&tabela) ret 4 -> al: *tabela = this; al = byte +0x438 (os bytes
#   altos do eax sao os do ponteiro; o chamador le movsx al). Prologo mov eax, [esp+4]; mov [eax], ecx (6 bytes).
# - 0x9037b0 UefaSeeding::add_intertoto_volunteer thiscall(clube) ret 4: sem lista (+0x43b), +0x43f = 0 e malloc(0x78)
#   (sem memoria, caixa uefa_seeding.cpp linha 0xf9 e fim); contagem (byte com sinal) abaixo de 30: o id do clube na
#   vaga e a contagem + 1. Prologo sub esp, 0x200 (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: a tabela as duas no mesmo ponteiro (o ponteiro e a contagem
# iguais); o voluntario lado a lado com a lista feita (sem ela so a original, que aloca).
SAVE = False
TELAS = True
ROTINAS = [('uefa_seeding_get_nation_table', 0x903590, [0x8b, 0x44, 0x24, 0x04, 0x89, 0x08]),
           ('uefa_seeding_add_intertoto_volunteer', 0x9037b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela do ranking da UEFA para as telas e os voluntários ingleses da Intertoto',
        'the UEFA seeding table for the screens and the English Intertoto volunteers',
        [('uefa_seeding_get_nation_table', 'UefaSeeding::get_nation_table', '0x903590', 'uefa_seeding.cpp', 'thiscall, ret 4',
          'dá às telas a tabela das nações do ranking da UEFA e o número de nações',
          'gives the screens the table of the UEFA seeding nations and the number of nations'),
         ('uefa_seeding_add_intertoto_volunteer', 'UefaSeeding::add_intertoto_volunteer', '0x9037b0', 'uefa_seeding.cpp', 'thiscall, ret 4',
          'põe um clube inglês na lista dos voluntários da Intertoto (até 30), criando a lista na primeira vez',
          'adds an English club to the Intertoto volunteer list (up to 30), making the list the first time')])
CAMPOS = []
INTERESSA = []
