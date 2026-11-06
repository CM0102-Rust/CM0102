# A Copa da UEFA: os inscritos (uefa_cup.cpp, em eurocomp\). Frente das competições (Agente D, 06/11/2026), carreira de
# 8 ligas (começa em janeiro de 2001).
# - 0x902b90 (UefaCup::build_entrants, 835 bytes; 2001-000, a Copa da UEFA de 2001/02): os clubes das vagas da
#   classificação da UEFA, completados até 118 com clubes sem copa de nações fracas, ordenados pela classe, e a lista dos
#   de classe 1 (os da Intertoto [inferência]). Um item de uma rotina grande.
# Igual no cm0102-1080.exe (a rotina inteira).
# Convenções conferidas (lição da 258a): thiscall, ret 0 (mov ecx, copa; call, no construtor 0x9022e0). Dentro: calloc
# 0x945501 (push 6; push 0xa1; add esp, 8 -> cdecl); 0x944e46 (push; add esp, 4 -> cdecl); qsort 0x945333 (4 pushes;
# add esp, 0x10 -> cdecl) com 0x522190; a caixa (aviso_em).
# Comparador: lado a lado a partir do mesmo estado (+0x14, +0x36..+0x39, +0x3e, +0xb2, [0xb67a34] e o +0x1db de cada
# clube, que os que completam recebem); as duas listas de cada lado comparadas e as da cópia liberadas.
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs9_uefa_inscritos', 0x902b90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da UEFA: os inscritos',
        'the UEFA Cup: the entrants',
        [('cqs9_uefa_inscritos', 'UefaCup::build_entrants', '0x902b90', 'uefa_cup.cpp', 'thiscall, ret 0',
          'regra: entram na Copa da UEFA os clubes que a classificação da UEFA pôs nela, cada um com a classe do seu coeficiente (os de 10 ou mais contam como da fase preliminar); se, descontada a metade dos da preliminar, não chegam a 118, completam os primeiros clubes sem copa europeia de nações sem a marca +0x11c e de coeficiente abaixo de 2000, com a classe 3 (e eles passam a ser da Copa da UEFA); passando de 118, os últimos saem; a lista é ordenada pela classe; com um número par de clubes da preliminar, os de classe 1 vão a uma lista à parte (com a classe 3) e a copa passa a ter os inscritos + 27 vagas (as 27 da Liga dos Campeões e da Intertoto que chegam depois [inferência]); com um número ímpar, a copa fica com 145 vagas e devolve 0',
          'rule: the UEFA Cup takes the clubs the UEFA ranking put in it, each with its coefficient class (those of 10 or more count as preliminary-round clubs); if, minus half of the preliminary ones, they are fewer than 118, the first clubs without a European cup from nations without the +0x11c mark and coefficient below 2000 complete it, with class 3 (and they become UEFA Cup clubs); beyond 118, the last ones leave; the list is sorted by class; with an even number of preliminary clubs, the class 1 ones go to a separate list (with class 3) and the cup gets entrants + 27 slots (the 27 coming later from the Champions League and the Intertoto [inference]); with an odd number, the cup keeps 145 slots and returns 0')])
CAMPOS = [('clube +0x1db copa europeia (id), +0x1df coeficiente (byte com sinal); id/valor; lê, e grava +0x1db em quem completa',
           'club +0x1db European cup (id), +0x1df coefficient (signed byte); id/value; reads, and writes +0x1db for fillers'),
          ('nação +0x11c marca, +0x8e coeficiente (word com sinal); valor; lê', 'nation +0x11c mark, +0x8e coefficient (signed word); value; reads'),
          ('copa +0x3e vagas, +0xb2 clubes da preliminar, +0x36/+0x38 contas da lista +0x14; valor; grava', 'cup +0x3e slots, +0xb2 preliminary clubs, +0x36/+0x38 counts of list +0x14; value; writes'),
          ('copa +0xa7 inscritos (6 bytes: clube, +4 classe, +5 situação), +0x14 lista dos de classe 1; ponteiro; grava', 'cup +0xa7 entrants (6 bytes: club, +4 class, +5 status), +0x14 class-1 list; pointer; writes'),
          ('[0x9cf6f0] a Copa da UEFA; id; lê', '[0x9cf6f0] the UEFA Cup; id; reads')]
INTERESSA = ['CMRTE']
