# 0x53c190 tira a pessoa do clube: os cortes da criação (0x541e60, 0x544970, 0x524d70) a chamam milhares de vezes antes
# da gravação do .qst da prova do save, e a carreira nas transferências e demissões. 0x53cd60 dá os olheiros do clube
# com os do time parceiro: só na carreira (SCOUT_MANAGER 0x7f7270, nas transferências e contratações); a criação do
# .qst não passa por ela (a mutação dela escapa da prova do save: ver mutacoes.py). Sem SEH, sem sorteio, sem variável
# local lida sem valor (a local da 0x53c190 só recebe a cópia de data que ninguém lê).
# Trocadas no lugar de 0x527240 (3,2 KB, sorteios, sem pseudocódigo do Ghidra), 0x532e00 e 0x533710 (jogadores falsos:
# a carreira e a prova do save não passam por elas); 0x53db40 e 0x539ae0 ficaram com o dbi_1/dbi_2 e a 0x53b4f0 já
# está no motor.rs.
SAVE = True
ROTINAS = [('tirar_pessoa_do_clube', 0x53c190, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('olheiros_do_clube', 0x53cd60, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a criação do jogo: tirar a pessoa do clube, os olheiros do clube com os do time parceiro',
        'game creation: removing a person from the club, the club scouts with the partner team ones',
        [('tirar_pessoa_do_clube', 'database_remove_staff_from_club', '0x53c190', 'Database.cpp', 'cdecl',
          'tira a pessoa da casa do clube que o cargo dela indica (gravando nela a data, quando vem, e, no treinador da seleção principal, também da sub-21) e, se pedido, zera o clube na pessoa; 0 se ela não estava na casa',
          'removes the person from the club slot her job points to (writing the date into her, when given, and, for the senior national team manager, from the under-21 side too) and, if asked, clears the club in the person; 0 if she was not in the slot'),
         ('olheiros_do_clube', 'database_get_club_scouts', '0x53cd60', 'Database.cpp', 'cdecl',
          'copia os 7 olheiros do clube para um vetor; com time parceiro (A/B), junta os dois até 7 e zera o resto',
          'copies the 7 club scouts into an array; with a partner team (senior/B), merges both up to 7 and clears the rest')])
CAMPOS = [('TClub +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): casas do clube; ponteiros; grava (tirar_pessoa_do_clube)', 'TClub +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): club slots; pointers; writes (tirar_pessoa_do_clube)'),
          ('TClub +0x53 nação; lê (tirar_pessoa_do_clube)', 'TClub +0x53 nation; reads (tirar_pessoa_do_clube)'),
          ('TClub +0x1b3 (7) olheiros; ponteiros; lê (olheiros_do_clube)', 'TClub +0x1b3 (7) scouts; pointers; reads (olheiros_do_clube)'),
          ('TStaff +0x24 seleção, +0x28 cargo na seleção, +0x29 data na seleção, +0x39 clube, +0x3d cargo, +0x3e data no clube; grava (tirar_pessoa_do_clube)', 'TStaff +0x24 national team, +0x28 national job, +0x29 national date, +0x39 club, +0x3d job, +0x3e club date; writes (tirar_pessoa_do_clube)')]
INTERESSA = ['SGE', 'CMRTE']
