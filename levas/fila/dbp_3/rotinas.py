# A pessoa rejeitada que sai das preferências (0x547c50: relações do elenco game_update_squad_relationships 0x5d6d10,
# diária; volta da lesão 0x630ee0; mídia 0x7414ac/0x75a340) e as pessoas favoritas copiadas num vetor (0x548090: a
# opinião da diretoria sobre o elenco 0x693850 e a reação dela à partida 0x6aaee0). Simulação: as mutações pela
# carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem sorteio, sem chamadas, sem variável
# local. A 0x547c50 roda nas relações do elenco (diária, laço pelos clubes): corre o risco da 0x547770 (233ª), de o
# desvio mudar o resíduo de pilha que uma original lê depois.
SAVE = False
ROTINAS = [('tirar_pessoa_rejeitada', 0x547c50, [0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2]),
           ('pessoas_favoritas', 0x548090, [0x8b, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('a pessoa rejeitada que sai das preferências e as pessoas favoritas num vetor',
        'removing a disliked person from the preferences and the favoured staff into an array',
        [('tirar_pessoa_rejeitada', 'staff_remove_disliked_staff', '0x547c50', 'Database.cpp', 'cdecl',
          'zera a primeira das três pessoas rejeitadas da pessoa (preferências +0x28..+0x30) que é o alvo',
          'clears the first of the three disliked staff of the person (preferences +0x28..+0x30) that is the target'),
         ('pessoas_favoritas', 'staff_get_favoured_staff', '0x548090', 'Database.cpp', 'cdecl',
          'zera o contador e copia as pessoas favoritas não nulas (preferências +0x1c..+0x24) para o vetor, contando',
          'clears the counter and copies the non-null favoured staff (preferences +0x1c..+0x24) into the array, counting them')])
CAMPOS = [('preferências (TStaff +0x65) +0x28..+0x30 pessoas rejeitadas; grava (tirar_pessoa_rejeitada)', 'preferences (TStaff +0x65) +0x28..+0x30 disliked staff; writes (tirar_pessoa_rejeitada)'),
          ('preferências (TStaff +0x65) +0x1c..+0x24 pessoas favoritas; lê (pessoas_favoritas)', 'preferences (TStaff +0x65) +0x1c..+0x24 favoured staff; reads (pessoas_favoritas)')]
INTERESSA = ['SGE', 'CMRTE']
