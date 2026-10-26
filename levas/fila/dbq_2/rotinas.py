# O clube rejeitado da pessoa (0x547e70: diário das pessoas game_daily_staff_update 0x5ce2d0, 0x5a5980,
# MANAGER_MANAGER 0x69c030, 0x7a4a30, 0x8c163d), o esfriar das relações pessoais (0x547f90: diário das pessoas) e a
# lista das pessoas rejeitadas (0x5480f0: diário das pessoas, MANAGER_MANAGER 0x693850, mídia, telas de pessoas).
# Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem variável
# local, sem chamadas além do sorteio. A 0x547e70 sorteia (3) só sem vaga e com `forcar`; a 0x547f90 sorteia uma vez
# por vaga ocupada (até 6): o comparador volta o estado do sorteio entre os lados e a cópia roda sem contar. A
# 0x5480f0 não sorteia; devolve o terceiro argumento (o eax da original), os chamadores leem só o byte. Nenhum
# chamador da 0x547e70 e da 0x547f90 lê o eax. As três rodam em laço quente (dentro do diário de cada pessoa): correm
# o risco da 0x547770 (233ª), de o desvio mudar o resíduo de pilha que uma original lê depois.
SAVE = False
ROTINAS = [('acrescentar_clube_rejeitado', 0x547e70, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('esmaecer_relacoes', 0x547f90, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('pessoas_rejeitadas', 0x5480f0, [0x8b, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('o clube rejeitado da pessoa, o esfriar das relações pessoais e a lista das pessoas rejeitadas',
        'the disliked club of a person, the fading of personal relationships and the list of disliked staff',
        [('acrescentar_clube_rejeitado', 'staff_add_disliked_club', '0x547e70', 'Database.cpp', 'cdecl',
          'põe o clube entre os três rejeitados da pessoa (preferências +0x10..+0x18): o clube já rejeita a pessoa (+0xa7..+0xaf), nada; favorito (+4..+0xc), deixa de ser; senão a primeira vaga; sem vaga e com o terceiro argumento, a vaga sorteada (3)',
          'puts the club among the three disliked clubs of the person (preferences +0x10..+0x18): the club already dislikes the person (+0xa7..+0xaf), nothing; favoured (+4..+0xc), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3)'),
         ('esmaecer_relacoes', 'staff_fade_staff_relationships', '0x547f90', 'Database.cpp', 'cdecl',
          'cada pessoa favorita (preferências +0x1c..+0x24) sai com o sorteio de (+0x59) + 15 dando 0; cada rejeitada (+0x28..+0x30), com o de 35 − (+0x5d)',
          'each favoured person (preferences +0x1c..+0x24) leaves when a random draw of (+0x59) + 15 gives 0; each disliked one (+0x28..+0x30), with 35 − (+0x5d)'),
         ('pessoas_rejeitadas', 'staff_get_disliked_staff', '0x5480f0', 'Database.cpp', 'cdecl',
          'copia as pessoas rejeitadas presentes (preferências +0x28..+0x30) para a saída, contando no byte do terceiro argumento (zerado sempre)',
          'copies the present disliked staff (preferences +0x28..+0x30) to the output, counting in the byte of the third argument (always cleared)')])
CAMPOS = [('preferências (TStaff +0x65) +4..+0xc clubes favoritos, +0x10..+0x18 clubes rejeitados; TClub +0xa7..+0xaf pessoas rejeitadas; lê e grava (acrescentar_clube_rejeitado)',
           'preferences (TStaff +0x65) +4..+0xc favoured clubs, +0x10..+0x18 disliked clubs; TClub +0xa7..+0xaf disliked staff; reads and writes (acrescentar_clube_rejeitado)'),
          ('preferências +0x1c..+0x30 pessoas favoritas e rejeitadas; TStaff +0x59, +0x5d; lê e grava (esmaecer_relacoes), lê (pessoas_rejeitadas)',
           'preferences +0x1c..+0x30 favoured and disliked staff; TStaff +0x59, +0x5d; reads and writes (esmaecer_relacoes), reads (pessoas_rejeitadas)')]
INTERESSA = ['SGE', 'CMRTE']
