# Acrescentar às preferências: a pessoa favorita do clube (0x547870: registros de clubes 0x445f45 e de pessoas
# 0x86dc30), a pessoa rejeitada do clube (0x547940: audiências das disputas 0x569ed0, mídia 0x755340, 0x7daf50) e a
# pessoa favorita da pessoa (0x547a10: relações do elenco 0x5d6d10 diária, registros de clubes, volta da lesão 0x630ee0,
# mídia e outras 9 chamadas). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real
# comparando. Sem SEH, sem variável local, sem retorno (nenhum chamador lê o eax; os dois que caem no epílogo, 0x7daf50
# e 0x86dc30, também não têm o eax lido por quem os chama). As três sorteiam (0x90cfc0, 3) só sem vaga e com `forcar`:
# o comparador volta o estado do sorteio entre os lados e a cópia roda sem contar. Em laço quente: a 0x547a10 (relações
# do elenco, diária); corre o risco da 0x547770 (233ª), de o desvio mudar o resíduo de pilha que uma original lê depois.
SAVE = False
ROTINAS = [('clube_acrescentar_pessoa_favorita', 0x547870, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('clube_acrescentar_pessoa_rejeitada', 0x547940, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('acrescentar_pessoa_favorita', 0x547a10, [0x53, 0x8b, 0x5c, 0x24, 0x08])]
DOCS = ('acrescentar às preferências: a pessoa favorita e a rejeitada do clube e a pessoa favorita da pessoa',
        'adding to the preferences: the favoured and the disliked person of a club and the favoured person of a person',
        [('clube_acrescentar_pessoa_favorita', 'club_add_favoured_staff', '0x547870', 'Database.cpp', 'cdecl',
          'põe a pessoa entre as três favoritas do clube (+0x9b..+0xa3): já favorita, nada; rejeitada (+0xa7..+0xaf), deixa de ser; senão a primeira vaga; sem vaga e com o terceiro argumento, a vaga sorteada (3)',
          'puts the person among the three favoured staff of the club (+0x9b..+0xa3): already favoured, nothing; disliked (+0xa7..+0xaf), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3)'),
         ('clube_acrescentar_pessoa_rejeitada', 'club_add_disliked_staff', '0x547940', 'Database.cpp', 'cdecl',
          'põe a pessoa entre as três rejeitadas do clube (+0xa7..+0xaf): já rejeitada, nada; favorita (+0x9b..+0xa3), deixa de ser; senão a primeira vaga; sem vaga e com o terceiro argumento, a vaga sorteada (3)',
          'puts the person among the three disliked staff of the club (+0xa7..+0xaf): already disliked, nothing; favoured (+0x9b..+0xa3), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3)'),
         ('acrescentar_pessoa_favorita', 'staff_add_favoured_staff', '0x547a10', 'Database.cpp', 'cdecl',
          'põe a outra pessoa entre as três favoritas da pessoa (preferências +0x1c..+0x24): já favorita, nada; rejeitada (+0x28..+0x30), deixa de ser; senão a primeira vaga; sem vaga e com o terceiro argumento, a vaga sorteada (3)',
          'puts the other person among the three favoured staff of the person (preferences +0x1c..+0x24): already favoured, nothing; disliked (+0x28..+0x30), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3)')])
CAMPOS = [('TClub +0x9b..+0xa3 pessoas favoritas, +0xa7..+0xaf pessoas rejeitadas; lê e grava (clube_acrescentar_pessoa_favorita, clube_acrescentar_pessoa_rejeitada)',
           'TClub +0x9b..+0xa3 favoured staff, +0xa7..+0xaf disliked staff; reads and writes (clube_acrescentar_pessoa_favorita, clube_acrescentar_pessoa_rejeitada)'),
          ('preferências (TStaff +0x65) +0x1c..+0x24 pessoas favoritas, +0x28..+0x30 pessoas rejeitadas; lê e grava (acrescentar_pessoa_favorita)',
           'preferences (TStaff +0x65) +0x1c..+0x24 favoured staff, +0x28..+0x30 disliked staff; reads and writes (acrescentar_pessoa_favorita)')]
INTERESSA = ['SGE', 'CMRTE']
