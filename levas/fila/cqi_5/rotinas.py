# A tabela das quatro divisões inglesas (o slot create_table de eng_prm.cpp, eng_second.cpp, eng_third.cpp e
# eng_first.cpp; Agente B da frente das competições, 03/11/2026): aloca a tabela, põe os clubes da divisão na ordem do
# banco, calcula o fundo de prêmios e paga o dinheiro da TV. Rodam uma vez cada, na criação da carreira (o construtor
# as chama direto: 0x57343f mov ecx, esi; call 0x574a20 -> thiscall, ret 0; idem nas outras), na carreira de 8 ligas.
# A 0x572bf0 (EngFirst) não aparece na docs/cobertura_competicoes.txt, mas roda (o contador "cqi_5 tabela cheia" chega a 4
# na carreira Rust de 8 ligas).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl;
# - 0x687430 (League::init_table_row): push 0; push 0; push clube; push eax (al = a linha, os bits de cima do id da
#   competição); mov ecx, esi -> thiscall, ret 0x10 (a cópia passa a linha limpa: a 0x687430 só lê o byte);
# - 0x682f70 (League::calc_prize_fund): push cota; mov ecx, esi -> thiscall, ret 4; devolve o fundo (+0x28);
# - 0x683010 (League::pay_tv_rights): push filtro; push valor; mov ecx, esi -> thiscall, ret 8;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original (aloca a tabela, paga e manda a notícia da TV), conferindo o plano feito antes: os clubes
# da divisão na ordem do banco nas linhas, +0x3e, a devolução e, cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
# Determinismo: percorre o banco na ordem; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('tabela_da_premier', 0x574a20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tabela_da_second', 0x576630, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tabela_da_third', 0x577f70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tabela_da_first', 0x572bf0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela das quatro divisões inglesas: os clubes, o fundo de prêmios e o dinheiro da TV',
 'the table of the four English divisions: the clubs, the prize fund and the TV money',
 [('tabela_da_premier', 'EngPrm::create_table', '0x574a20', 'eng_prm.cpp', 'thiscall, ret 0',
   'a Premier tem 20 clubes; o fundo de prêmios parte de 250.000 por posição; a TV paga 12.000.000 a cada clube '
   '(5.000.000 até 1999)',
   'the Premier has 20 clubs; the prize fund starts at 250,000 per place; TV pays 12,000,000 to each club '
   '(5,000,000 up to 1999)'),
  ('tabela_da_second', 'EngSecond::create_table', '0x576630', 'eng_second.cpp', 'thiscall, ret 0',
   'a Second tem 24 clubes; fundo de 20.000 por posição; a TV paga 2.500.000 a cada clube (300.000 até 1999)',
   'the Second has 24 clubs; prize fund of 20,000 per place; TV pays 2,500,000 to each club (300,000 up to 1999)'),
  ('tabela_da_third', 'EngThird::create_table', '0x577f70', 'eng_third.cpp', 'thiscall, ret 0',
   'a Third tem 24 clubes; fundo de 10.000 por posição; a TV paga 1.000.000 a cada clube (200.000 até 1999)',
   'the Third has 24 clubs; prize fund of 10,000 per place; TV pays 1,000,000 to each club (200,000 up to 1999)'),
  ('tabela_da_first', 'EngFirst::create_table', '0x572bf0', 'eng_first.cpp', 'thiscall, ret 0',
   'a First tem 24 clubes; fundo de 70.000 por posição; a TV paga pela divisão de onde o clube veio: 5.877.777 aos '
   'que caíram da Premier e 4.099.999 aos que já estavam na First e aos que subiram da Second (1.000.000 a todos até '
   '1999)',
   'the First has 24 clubs; prize fund of 70,000 per place; TV pays by the division the club came from: 5,877,777 '
   'to those relegated from the Premier and 4,099,999 to those already in the First and those promoted from the '
   'Second (1,000,000 to all up to 1999)')])
CAMPOS = [('liga +0x3e clubes; valor; grava (as quatro)', 'league +0x3e clubs; value; writes (all four)'),
          ('liga +0xb1 tabela (linhas de 0x3b bytes, +0 clube); ponteiro; grava (as quatro, pela 0x687430)',
           'league +0xb1 table (0x3b-byte rows, +0 club); pointer; writes (all four, through 0x687430)'),
          ('liga +0x28 fundo de prêmios; valor; grava; +0xe1 = 1 [sem nome]; valor; grava (as quatro)',
           'league +0x28 prize fund; value; writes; +0xe1 = 1 [unnamed]; value; writes (all four)'),
          ('clube +0x57 competição (id no +0); ponteiro; lê (as quatro); +0x5b competição anterior; lê pela 0x683010 '
           '(tabela_da_first)',
           'club +0x57 competition (id at +0); pointer; reads (all four); +0x5b previous competition; reads through '
           '0x683010 (tabela_da_first)'),
          ('[0xae2c92] ano do jogo; valor; lê (as quatro)', '[0xae2c92] game year; value; reads (all four)')]
INTERESSA = ['CMRTE']
