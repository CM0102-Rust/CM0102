# Tirar das preferências: o clube favorito da pessoa (0x547f50, no diário das pessoas game_daily_staff_update
# 0x5ce2d0, quando o apego ao clube acaba), a pessoa favorita do clube (0x547bd0, só nas audiências das disputas
# 0x569ed0) e a pessoa favorita da pessoa (0x547c10: disputas, relações do elenco 0x5d6d10 diária, volta da lesão
# 0x630ee0, MANAGER_MANAGER 0x697a7a, mídia 0x75a340). Simulação: as mutações pela carreira até 2002-06-01 e a
# confirmação no jogo real comparando. Sem SEH, sem sorteio, sem chamadas, sem variável local. Em laço quente: a
# 0x547f50 (dentro do diário de cada pessoa) e a 0x547c10 (relações do elenco, diária); correm o risco da 0x547770
# (233ª), de o desvio mudar o resíduo de pilha que uma original lê depois.
SAVE = False
ROTINAS = [('tirar_clube_favorito', 0x547f50, [0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2]),
           ('clube_tirar_pessoa_favorita', 0x547bd0, [0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2]),
           ('tirar_pessoa_favorita', 0x547c10, [0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2])]
DOCS = ('tirar das preferências: o clube favorito da pessoa, a pessoa favorita do clube e a pessoa favorita da pessoa',
        'removing from the preferences: the favoured club of a person, the favoured person of a club and the favoured person of a person',
        [('tirar_clube_favorito', 'staff_remove_favoured_club', '0x547f50', 'Database.cpp', 'cdecl',
          'zera o primeiro dos três clubes favoritos da pessoa (preferências +4..+0xc) que é o clube',
          'clears the first of the three favoured clubs of the person (preferences +4..+0xc) that is the club'),
         ('clube_tirar_pessoa_favorita', 'club_remove_favoured_staff', '0x547bd0', 'Database.cpp', 'cdecl',
          'zera a primeira das três pessoas favoritas do clube (+0x9b..+0xa3) que é a pessoa',
          'clears the first of the three favoured staff of the club (+0x9b..+0xa3) that is the person'),
         ('tirar_pessoa_favorita', 'staff_remove_favoured_staff', '0x547c10', 'Database.cpp', 'cdecl',
          'zera a primeira das três pessoas favoritas da pessoa (preferências +0x1c..+0x24) que é o alvo',
          'clears the first of the three favoured staff of the person (preferences +0x1c..+0x24) that is the target')])
CAMPOS = [('preferências (TStaff +0x65) +4..+0xc clubes favoritos; grava (tirar_clube_favorito)', 'preferences (TStaff +0x65) +4..+0xc favoured clubs; writes (tirar_clube_favorito)'),
          ('TClub +0x9b/+0x9f/+0xa3: pessoas favoritas do clube; ponteiros; grava (clube_tirar_pessoa_favorita)', 'TClub +0x9b/+0x9f/+0xa3: favoured staff of the club; pointers; writes (clube_tirar_pessoa_favorita)'),
          ('preferências (TStaff +0x65) +0x1c..+0x24 pessoas favoritas; grava (tirar_pessoa_favorita)', 'preferences (TStaff +0x65) +0x1c..+0x24 favoured staff; writes (tirar_pessoa_favorita)')]
INTERESSA = ['SGE', 'CMRTE']
