# A escalação guardada do clube (tactics.cpp): Tactics::remove_player_from_selection 0x896c60 (pelos contratos, pelas
# saídas do elenco 0x690730, pela escalação automática 0x6dbc50, pelas seleções, pela aposentadoria 0x7aace0 e pelas
# transferências; a 257ª contou "vaga extra apagada" 65536 vezes, a maior parte por ela), Tactics::swap_selection_slots
# 0x8973e0 e Tactics::has_selection_undo 0x897250 (as duas só pela tela do elenco: passageiras). Iguais no
# cm0102-1080.exe; sem SEH; sem x87; o quadro de 0x20c/0x200 bytes é só da caixa de erro (a original). Comparadores lado
# a lado: a 0x896c60 e a 0x8973e0 pela devolução, pela linha do clube na tabela tática (0x91 bytes: as vagas, as extras
# e os batedores) e pelas 16 cópias para desfazer com a conta (this+0x30..+0x5f0); a 0x897250 pela devolução.
SAVE = False
ROTINAS = [('tirar_da_escalacao', 0x896c60, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('trocar_vagas_da_escalacao', 0x8973e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tem_copia_da_escalacao', 0x897250, [0x8a, 0x81, 0xf0, 0x05, 0x00, 0x00])]
DOCS = ('a escalação guardada do clube: tirar o jogador da escalação, trocar duas vagas e a cópia para desfazer',
        'the stored club selection: removing the player from the selection, swapping two slots and the undo copy',
        [('tirar_da_escalacao', 'Tactics::remove_player_from_selection', '0x896c60', 'tactics.cpp', 'thiscall, ret 0x10',
          'esvazia (−1) as vagas da escalação com a pessoa; com papéis, também as duas vagas extras e os batedores de bola parada; as cópias para desfazer do clube e do time somem; 1 se a tirou de uma vaga ou de bola parada',
          'empties (−1) the selection slots holding the person; with roles, also the two extra slots and the set piece takers; the undo copies of the club and team are dropped; 1 when removed from a slot or set piece'),
         ('trocar_vagas_da_escalacao', 'Tactics::swap_selection_slots', '0x8973e0', 'tactics.cpp', 'thiscall, ret 0x10',
          'troca os ids de duas vagas (0..19) da escalação e solta as cópias para desfazer do clube e do time',
          'swaps the ids of two selection slots (0..19) and drops the undo copies of the club and team'),
         ('tem_copia_da_escalacao', 'Tactics::has_selection_undo', '0x897250', 'tactics.cpp', 'thiscall, ret 8',
          '1 se o humano da vez tem a cópia para desfazer da escalação deste clube e time',
          '1 when the current human holds the selection undo copy of this club and team')])
CAMPOS = [('Tactics +0x601 tabela tática (linhas de 0x91 bytes: +2 as 20 vagas da escalação, ids, −1 vazia; +0x56/+0x5a vagas extras; +0x5e batedores); ids; lê e grava (tirar_da_escalacao, trocar_vagas_da_escalacao)',
           'Tactics +0x601 tactic table (0x91-byte rows: +2 the 20 selection slots, ids, −1 empty; +0x56/+0x5a extra slots; +0x5e set piece takers); ids; reads and writes (tirar_da_escalacao, trocar_vagas_da_escalacao)'),
          ('Tactics +0x30 cópias para desfazer (16 × 0x5c bytes: +0 clube, +4 time, +8 a cópia), +0x5f0 conta (byte); ponteiros/valores; lê e grava +8 e a conta (tirar_da_escalacao, trocar_vagas_da_escalacao), lê (tem_copia_da_escalacao)',
           'Tactics +0x30 undo copies (16 × 0x5c bytes: +0 club, +4 team, +8 the copy), +0x5f0 count (byte); pointers/values; reads and writes +8 and the count (tirar_da_escalacao, trocar_vagas_da_escalacao), reads (tem_copia_da_escalacao)'),
          ('pessoa +0 id; [0xb74c26 + 0x300 × (i16)[0xb77c7a]] humano da vez; [0xae2364] conta de pessoas; ids; lê',
           'staff +0 id; [0xb74c26 + 0x300 × (i16)[0xb77c7a]] current human; [0xae2364] staff count; ids; reads')]
INTERESSA = ['CMRTE']
