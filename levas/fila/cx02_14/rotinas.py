# A lista de inscritos das copas com grupos no jogo salvo, leitura (gre_cup.cpp, ire_lge_cup.cpp, ita_c_cup.cpp; leitor
# cx02, rodada 5, 24/11/2026). Item do SAVE (CARGA). Rodam quando a liga e carregada do arquivo temporario (a virtual 0x20
# chamada pela carga da League, com o FILE aberto em CM3_TEMP); so com um save que tenha a Grecia, a Irlanda ou a Italia
# (com a Serie C): o FeriasTesteSave (so a Inglaterra) nao as chama. A prova pede um save com essas ligas.
# - 0x5e9df0 GreCup::load_extra_data, 0x638260 IreLgeCup::load_extra_data e 0x63e3c0 ItaCCup::load_extra_data
#   thiscall(arquivo) ret 4 -> 0/1: instrucao por instrucao iguais menos o arquivo-fonte e as tres linhas das caixas;
#   fread 0x945bfe (cdecl) da contagem (+0xf2, 4 bytes), malloc 0x944e46 (cdecl) de contagem x 4 em +0xee, fread dos ids
#   e cada id vira o clube ([0xae23b4] + id x 0x245; -1 vira 0). O auxiliar load_group_cup_entrants fica aqui (o
#   cx02_15 e o cx02_16 o usam). Prologo sub esp, 0x200.
# Iguais no cm0102-1080.exe.
# Comparadores: a original le; o arquivo volta a posicao (ftell/fseek) e a copia le de novo numa lista propria; os
# retornos, as contagens, as posicoes e os clubes iguais; a lista da copia e liberada.
SAVE = True
CARGA = True
ROTINAS = [('gre_cup_load_extra_data', 0x5e9df0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ire_lge_cup_load_extra_data', 0x638260, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_c_cup_load_extra_data', 0x63e3c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a lista de inscritos das copas com grupos no jogo salvo (leitura)', "the group cups' entrant list in the saved game (load)",
        [('gre_cup_load_extra_data', 'GreCup::load_extra_data', '0x5e9df0', 'gre_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Copa da Grécia o número de inscritos e os ids dos clubes, que voltam a ser clubes (−1, vaga vazia); leitura curta ou falta de memória: a caixa de erro e 0',
          "reads from the Greek Cup temporary file the entrant count and the clubs' ids, which become clubs again (−1, an empty slot); a short read or no memory: the error box and 0"),
         ('ire_lge_cup_load_extra_data', 'IreLgeCup::load_extra_data', '0x638260', 'ire_lge_cup.cpp', 'thiscall, ret 4',
          'o mesmo para a Copa da Liga irlandesa',
          'the same for the Irish League Cup'),
         ('ita_c_cup_load_extra_data', 'ItaCCup::load_extra_data', '0x63e3c0', 'ita_c_cup.cpp', 'thiscall, ret 4',
          'o mesmo para a Copa da Série C',
          'the same for the Serie C cup')])
CAMPOS = []
INTERESSA = []
