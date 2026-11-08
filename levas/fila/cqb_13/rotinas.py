# O calendario (fix_man.cpp):
# - 0x5ae380 (FIXTURE_MANAGER::resolve_same_day_clash): pela FIXTURE_MANAGER::add_fixture 0x5aa680 (original) quando o
#   jogo novo cai no dia de outro (gravidade 0x14); 7 chamadas na carreira automatica ate 2002-06-01
#   (docs/cobertura_simulacao.txt, 17/10). Decide qual dos dois sai do dia e para onde vai: a data dos jogos, lida por
#   toda a simulacao.
# - 0x5aee40 (get_club_first_fixture_year) e 0x5aee60 (get_club_last_fixture_year): pela tela da historia do clube
#   (0x45f540/0x45f4a0; passageiras, 0 chamadas na carreira).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   As proprias: 0x5ae380 thiscall (push esi = conflitos; push ebp = jogo; mov ecx, edi em 0x5aa8c3), 2 args, ret 8;
#   0x5aee40/0x5aee60 thiscall (mov ecx, [0xae2a58]; push edi = &id em 0x45f536/0x45f496; o this nao e lido), 1 arg,
#   ret 4, ax.
#   0x54a110 data_padrao: thiscall (lea ecx, data; sem pushes em 0x5ae3a1/0x5ae3ac). 0x68d1b0 proximo_da_lista: thiscall
#   (mov ecx, [esp + 0x2e8] = conflitos em 0x5ae3bc), 0 args. 0x549ef0 data_nova: thiscall (lea ecx, tmp em 0x5ae41b),
#   2 args (push [+0x28]; push [+0x2a]: dia = +0x2a, ano = +0x28; os bytes de cima nao contam). 0x417c10 copia:
#   thiscall (lea ecx, A em 0x5ae430), 2 args (push o retorno; push &tmp2). 0x54a3f0 dia_da_semana: thiscall (lea ecx,
#   A), 0 args, ax (movsx). 0x54a2d0 ajustar_semana: thiscall (lea ecx, A em 0x5ae477), 2 args (push &B; push alvo).
#   0x53b900 e_selecao: cdecl (add esp, 4 em 0x5ae4a0/0x5ae4b4). 0x5abc30 tirar_jogo_do_calendario: thiscall (push ebx;
#   mov ecx, ebp em 0x5ae4f4), 1 arg. 0x5aa680 add_fixture: thiscall (mov ecx, ebp), 2 args (push 0; push jogo), ret 8.
#   0x5ad780 find_fixture_clashes: thiscall (mov ecx, ebp em 0x5ae52c/0x5ae5a1), 3 args (push 0; push &gravidade; push
#   jogo), ret 0xc, a lista nova ou 0. 0x68d040 (destruir a lista): thiscall (mov ecx, esi), 0 args. 0x944c94 delete:
#   cdecl (push esi; add esp, 4), 1 arg. 0x5ab510 find_new_fixture_date: thiscall (push &copia; mov ecx, ebp), 1 arg.
#   0x5aac70 noticia_do_jogo_remarcado: thiscall (mov ecx, ebp), 3 args (push 0; push causa; push &copia), ret 0xc. A
#   caixa pelo aviso_em (cdecl, como em 0x5ae612..0x5ae654).
# A 0x5ae380 tem quadro SEH (as duas datas locais), sem destrutor no caminho normal. O +0x3a do jogo (o jogo fixo) e
# lido depois do push do delete (mov al, [esp + 0x62] com o push pendente). Comparadores: a 0x5ae380 em ensaio (a
# copia para na decisao, antes do calendario e da caixa; lado a lado no cabecalho da lista: +2 o comeco e +0xa o
# cursor, que a 0x68d1b0 anda) e a original depois (o calendario, a lista alocada e a noticia nao rodam duas vezes); as
# duas dos anos comparam o valor (compare_value!). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('conflito_de_jogos_no_mesmo_dia', 0x5ae380, [0x6a, 0xff, 0x68, 0x16, 0x98, 0x95, 0x00]),
           ('primeiro_ano_com_jogos', 0x5aee40, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x5c, 0x2a, 0xae, 0x00]),
           ('ultimo_ano_com_jogos', 0x5aee60, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x5c, 0x2a, 0xae, 0x00])]
DOCS = ('o calendário: o conflito de dois jogos no mesmo dia e o primeiro e o último ano com jogos de um clube',
        "the fixture calendar: the clash of two fixtures on one day and a club's first and last year with fixtures",
        [('conflito_de_jogos_no_mesmo_dia', 'FIXTURE_MANAGER::resolve_same_day_clash', '0x5ae380', 'fix_man.cpp', 'thiscall',
          'o jogo novo e o primeiro do mesmo dia: fica o de prioridade maior (a da competição, +21 com seleção mandante; empate pela reputação do mandante), desde que o outro ainda não tenha sido jogado; o que sai vai para o dia da semana alvo pela data do outro (os dias 0 e 5 da semana vão a 6, 1 e 3 a 2, o 2 a 1, 4 e 6 a 5) ou, com conflito ali, para outra data, com a notícia da remarcação; jogo fixo com conflito não muda (devolve 0)',
          'the new fixture and the first one on the same day: the higher priority stays (the competition one, +21 with a national team at home; ties by the home club reputation), as long as the other was not played yet; the one that leaves goes to the target weekday from the other date (weekdays 0 and 5 go to 6, 1 and 3 to 2, 2 to 1, 4 and 6 to 5) or, with a clash there, to another date, with the rescheduling news; a fixed fixture with a clash does not move (returns 0)'),
         ('primeiro_ano_com_jogos', 'FIXTURE_MANAGER::get_club_first_fixture_year', '0x5aee40', 'fix_man.cpp', 'thiscall',
          'o primeiro ano com jogos do clube: 2001, ou 2002 pelo bit 0 da palavra dele em [0xae2a5c]',
          "the club's first year with fixtures: 2001, or 2002 by bit 0 of its word in [0xae2a5c]"),
         ('ultimo_ano_com_jogos', 'FIXTURE_MANAGER::get_club_last_fixture_year', '0x5aee60', 'fix_man.cpp', 'thiscall',
          'o último ano com jogos do clube: a palavra dele em [0xae2a5c] sem o bit 0, ou o ano corrente',
          "the club's last year with fixtures: its word in [0xae2a5c] without bit 0, or the current year")])
CAMPOS = [('jogo (0x4f bytes) +0x14 competição, +0x1c mandante, +0x28 ano, +0x2a dia, +0x3a jogo fixo, +0x43 resultado (0xff sem jogo); ponteiros/valores; lê e grava a data na cópia (conflito_de_jogos_no_mesmo_dia)',
           'fixture (0x4f bytes) +0x14 competition, +0x1c home club, +0x28 year, +0x2a day, +0x3a fixed, +0x43 result (0xff unplayed); pointers/values; reads, writes the date in the copy (conflito_de_jogos_no_mesmo_dia)'),
          ('competição +0x69 prioridade; valor; lê (conflito_de_jogos_no_mesmo_dia)', 'competition +0x69 priority; value; reads (conflito_de_jogos_no_mesmo_dia)'),
          ('clube +0x80 reputação; valor; lê (conflito_de_jogos_no_mesmo_dia)', 'club +0x80 reputation; value; reads (conflito_de_jogos_no_mesmo_dia)'),
          ('lista (LLIST) +2 começo, +0xa cursor; ponteiros; lê e grava o cursor (conflito_de_jogos_no_mesmo_dia)',
           'list (LLIST) +2 head, +0xa cursor; pointers; reads and writes the cursor (conflito_de_jogos_no_mesmo_dia)'),
          ('[0xae2a5c] + 2 × id do clube: anos com jogos (bit 0 o primeiro, o resto o último × 2); valor; lê (primeiro_ano_com_jogos, ultimo_ano_com_jogos)',
           '[0xae2a5c] + 2 × club id: years with fixtures (bit 0 the first, the rest the last × 2); value; reads (primeiro_ano_com_jogos, ultimo_ano_com_jogos)'),
          ('[0xae2c92] ano corrente; valor; lê (ultimo_ano_com_jogos)', '[0xae2c92] current year; value; reads (ultimo_ano_com_jogos)')]
INTERESSA = []
