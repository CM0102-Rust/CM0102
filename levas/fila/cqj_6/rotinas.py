# A tabela das três divisões francesas (o slot create_table de fra_first.cpp, fra_second.cpp e fra_third.cpp;
# Agente B da frente das competições, 03/11/2026): aloca a tabela, põe os clubes da divisão na ordem do banco, o fundo
# de prêmios e a TV. Rodam uma vez cada, na criação da carreira de 8 ligas (o construtor as chama direto: mov ecx,
# esi; call -> thiscall, ret 0).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x944e46 (malloc): push n × 0x3b; call; add esp, 4 -> cdecl;
# - 0x687430 (League::init_table_row): push 0; push 0; push clube; push eax (al = a linha); mov ecx, esi -> thiscall,
#   ret 0x10 (a cópia passa a linha limpa: a 0x687430 só lê o byte);
# - 0x682f70 (League::calc_prize_fund): push cota; mov ecx, esi -> thiscall, ret 4;
# - 0x683010 (League::pay_tv_rights): push filtro; push valor; mov ecx, esi -> thiscall, ret 8 (na segunda, o filtro
#   é o registro de competição [0xae23d0] + id × 0x6b);
# - a caixa de erro: aviso_em.
# Comparador: só a original (aloca, paga e manda a notícia da TV), conferindo o plano feito antes (os clubes da
# divisão na ordem do banco, +0x3e, a devolução, +0xe1 e +0x28).
# Determinismo: percorre o banco na ordem; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('tabela_da_franca_1', 0x5bc2b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tabela_da_franca_2', 0x5bef10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tabela_da_franca_3', 0x5c05d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela das três divisões francesas: os clubes, o fundo de prêmios e a TV pela divisão de origem',
 'the table of the three French divisions: the clubs, the prize fund and TV money by previous division',
 [('tabela_da_franca_1', 'FraFirst::create_table', '0x5bc2b0', 'fra_first.cpp', 'thiscall, ret 0',
   'a primeira tem 18 clubes na temporada 2001 e 20 depois; fundo de 200.000 por posição; a TV paga 6.000.000 a '
   'cada clube',
   'the top division has 18 clubs in the 2001 season and 20 afterwards; prize fund of 200,000 per place; TV pays '
   '6,000,000 to each club'),
  ('tabela_da_franca_2', 'FraSecond::create_table', '0x5bef10', 'fra_second.cpp', 'thiscall, ret 0',
   'a segunda tem 20 clubes; sem fundo de prêmios; a TV paga pela divisão de onde o clube veio: 2.000.000 aos que '
   'caíram da primeira e 1.500.000 aos que ficaram e aos que subiram do National',
   'the second has 20 clubs; no prize fund; TV pays by the division the club came from: 2,000,000 to those '
   'relegated from the top flight and 1,500,000 to those who stayed and those promoted from the National'),
  ('tabela_da_franca_3', 'FraThird::create_table', '0x5c05d0', 'fra_third.cpp', 'thiscall, ret 0',
   'o National tem 20 clubes; fundo de 20.000 por posição; a TV paga 200.000 a cada clube',
   'the National has 20 clubs; prize fund of 20,000 per place; TV pays 200,000 to each club')])
CAMPOS = [('liga +0x3e clubes; +0xb1 tabela (linhas de 0x3b bytes, +0 clube); valor e ponteiro; grava (as três)',
           'league +0x3e clubs; +0xb1 table (0x3b-byte rows, +0 club); value and pointer; writes (all three)'),
          ('liga +0x28 fundo de prêmios; +0xe1 modo do prêmio (1 = pela tabela, 2 = nenhum); valor; grava (as três)',
           'league +0x28 prize fund; +0xe1 prize mode (1 = by table, 2 = none); value; writes (all three)'),
          ('liga +0x40 ano da liga; valor; lê; +0xa7 = −1; valor; grava (tabela_da_franca_1)',
           'league +0x40 league year; value; reads; +0xa7 = −1; value; writes (tabela_da_franca_1)'),
          ('clube +0x57 competição (id no +0); ponteiro; lê; +0x5b competição anterior; lê pela 0x683010',
           'club +0x57 competition (id at +0); pointer; reads; +0x5b previous competition; reads through 0x683010')]
INTERESSA = ['CMRTE']
