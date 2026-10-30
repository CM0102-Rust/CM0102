# A escalação guardada na tabela tática (tactics.cpp): limpar uma vaga da escalação (Tactics::clear_selection_slot
# 0x896e90, pela convocação da seleção NationalTeams::select_squad 0x76d261/0x76d403 e por telas) e o batedor de bola
# parada (Tactics::set_set_piece_taker 0x897bb0, pela 0x896c60 da escalação do computador, que tira o batedor que
# deixou de jogar, e pela tela de táticas). Item de duas rotinas: as outras da tactics.cpp que faltam (capitão,
# pênaltis, troca de vagas) e o resto da família só rodam por telas, montam notícias na pilha (0x6a3770 das
# expectativas, a convocação 0x76c120) ou não rodam na carreira inglesa. Sem SEH, sem sorteio, sem leitura de lixo; a
# linha tática vem da 0x8982b0 (já portada, consulta pura). Clube nulo ou índice fora: a cópia chama a original
# (caixa). Iguais no cm0102-1080.exe. Comparadores: lado a lado pela linha da escalação e pelos espaços guardados
# (0x896e90) e pela linha dos batedores (0x897bb0).
SAVE = False
ROTINAS = [('limpar_vaga_da_escalacao', 0x896e90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('marcar_batedor_de_bola_parada', 0x897bb0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a escalação guardada: limpar uma vaga e o batedor de bola parada',
        'the stored selection: clearing a slot and the set piece taker',
        [('limpar_vaga_da_escalacao', 'Tactics::clear_selection_slot', '0x896e90', 'tactics.cpp', 'thiscall, ret 0xc',
          'a vaga da escalação do clube na tabela tática fica vazia (−1) e os espaços guardados do mesmo clube e time são soltos',
          'the club selection slot in the tactics table becomes empty (−1) and the stored slots of the same club and team are released'),
         ('marcar_batedor_de_bola_parada', 'Tactics::set_set_piece_taker', '0x897bb0', 'tactics.cpp', 'thiscall, ret 0x14',
          'grava o batedor de uma bola parada (primeira ou segunda escolha); sem pessoa, limpa e a segunda escolha sobe para a primeira',
          'stores the taker of a set piece (first or second choice); without a person, clears it and the second choice moves up to the first')])
CAMPOS = [('Tactics +0x601: a tabela tática (linhas de 0x91 bytes, linha da 0x8982b0): +2 as 20 vagas da escalação (id da pessoa ou −1), +0x5e os batedores (5 bolas paradas × 2 escolhas); id; lê e grava (limpar_vaga_da_escalacao, marcar_batedor_de_bola_parada)',
           'Tactics +0x601: the tactics table (0x91-byte rows, row from 0x8982b0): +2 the 20 selection slots (person id or −1), +0x5e the takers (5 set pieces × 2 choices); id; reads and writes (limpar_vaga_da_escalacao, marcar_batedor_de_bola_parada)'),
          ('Tactics +0x30: 16 espaços guardados de 0x5c bytes (+0 clube, +4 time, +8 ocupado) e +0x5f0 a conta; lê e grava 0 / −1 (limpar_vaga_da_escalacao)',
           'Tactics +0x30: 16 stored slots of 0x5c bytes (+0 club, +4 team, +8 used) and +0x5f0 the count; reads and writes 0 / −1 (limpar_vaga_da_escalacao)')]
INTERESSA = ['CMRTE', 'SGE']
