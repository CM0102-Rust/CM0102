# A tabela das divisões italianas (o slot create_table de ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp e
# ita_ser_c1b.cpp; Agente B da frente das competições, 03/11/2026): aloca a tabela, põe os clubes da divisão na ordem
# do banco e, na A e na B, o fundo de prêmios (e a TV na A). Rodam uma vez cada, na criação da carreira de 8 ligas
# (o construtor as chama direto: mov ecx, esi; call -> thiscall, ret 0).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl;
# - 0x687430 (League::init_table_row): push 0; push 0; push clube; push eax (al = a linha, os bits de cima do id da
#   competição); mov ecx, esi/edi -> thiscall, ret 0x10 (a cópia passa a linha limpa: a 0x687430 só lê o byte);
# - 0x682f70 (League::calc_prize_fund): push cota; mov ecx, esi -> thiscall, ret 4;
# - 0x683010 (League::pay_tv_rights): push 0; push 0x4c4b40; mov ecx, esi -> thiscall, ret 8 (só na A);
# - a caixa de erro: aviso_em.
# Comparador: só a original (aloca; a A paga e manda notícia), conferindo o plano feito antes (os clubes da divisão
# na ordem do banco nas linhas, +0x3e, a devolução e, na A e na B, +0xe1 = 1 e +0x28 = o fundo da cota).
# Determinismo: percorre o banco na ordem; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('tabela_da_serie_a', 0x641510, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tabela_da_serie_b', 0x646780, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tabela_da_c1a', 0x64b670, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tabela_da_c1b', 0x64f820, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela das divisões italianas: os clubes, o fundo de prêmios e a TV da Série A',
 'the table of the Italian divisions: the clubs, the prize fund and Serie A TV money',
 [('tabela_da_serie_a', 'ItaSerA::create_table', '0x641510', 'ita_ser_a.cpp', 'thiscall, ret 0',
   'a Série A tem 18 clubes; o fundo de prêmios parte de 250.000 por posição (pago pela tabela no fim da liga); a '
   'TV paga 5.000.000 a cada clube',
   'Serie A has 18 clubs; the prize fund starts at 250,000 per place (paid by table position when the league ends); '
   'TV pays 5,000,000 to each club'),
  ('tabela_da_serie_b', 'ItaSerB::create_table', '0x646780', 'ita_ser_b.cpp', 'thiscall, ret 0',
   'a Série B tem 20 clubes; fundo de 70.000 por posição; sem TV',
   'Serie B has 20 clubs; prize fund of 70,000 per place; no TV money'),
  ('tabela_da_c1a', 'ItaSerC1a::create_table', '0x64b670', 'ita_ser_c1a.cpp', 'thiscall, ret 0',
   'a Série C1/A tem 18 clubes; sem fundo de prêmios nem TV',
   'Serie C1/A has 18 clubs; no prize fund and no TV money'),
  ('tabela_da_c1b', 'ItaSerC1b::create_table', '0x64f820', 'ita_ser_c1b.cpp', 'thiscall, ret 0',
   'a Série C1/B tem 18 clubes; sem fundo de prêmios nem TV',
   'Serie C1/B has 18 clubs; no prize fund and no TV money')])
CAMPOS = [('liga +0x3e clubes; +0xb1 tabela (linhas de 0x3b bytes, +0 clube); valor e ponteiro; grava (as quatro)',
           'league +0x3e clubs; +0xb1 table (0x3b-byte rows, +0 club); value and pointer; writes (all four)'),
          ('liga +0x28 fundo de prêmios; +0xe1 modo do prêmio (1 = pela tabela, 2 = nenhum, 3 = pela competição, '
           'pela 0x682e10); valor; grava (tabela_da_serie_a, tabela_da_serie_b)',
           'league +0x28 prize fund; +0xe1 prize mode (1 = by table, 2 = none, 3 = by the competition, through '
           '0x682e10); value; writes (tabela_da_serie_a, tabela_da_serie_b)'),
          ('clube +0x57 competição (id no +0); ponteiro; lê (as quatro)',
           'club +0x57 competition (id at +0); pointer; reads (all four)')]
INTERESSA = ['CMRTE']
