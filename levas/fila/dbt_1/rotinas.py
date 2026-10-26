# Do Database.cpp, o clube e o seu time B, consultas das telas: quantos fisioterapeutas (0x53c9b0: telas da pessoa,
# staff_screens 0x8740c8/0x87412b), se o clube é time B (0x5448c0: tela da pessoa 0x87272d) e as pessoas fora do
# elenco num vetor (0x544c70: aba de informações da tela do clube, 0x4616fa). Simulação (SAVE = False). Sem SEH, sem
# sorteio; as locais de 0x200 só servem à caixa do clube nulo (fica com a original); a local `eh_a` da 0x5448c0 só é
# lida com parceiro, quando a 0x540a50 já a escreveu. A 0x540a50 (time B, já em Rust) é chamada pelo endereço do jogo.
SAVE = False
ROTINAS = [('contar_fisioterapeutas', 0x53c9b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('clube_e_time_b', 0x5448c0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('pessoas_fora_do_elenco', 0x544c70, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o clube e o seu time B: os fisioterapeutas, se é time B e as pessoas fora do elenco',
        'the club and its B team: physios, whether it is a B team and the non-playing staff',
        [('contar_fisioterapeutas', 'database_count_club_physios', '0x53c9b0', 'Database.cpp', 'cdecl',
          'quantos dos 3 fisioterapeutas (+0x1cf) do clube e do time B ligado são pessoas válidas; em `al`',
          'how many of the 3 physios (+0x1cf) of the club and of its linked B team are valid people; in `al`'),
         ('clube_e_time_b', 'club_is_b_team', '0x5448c0', 'Database.cpp', 'cdecl',
          '1 se o clube tem parceiro (0x540a50 com a marca) e é o B dele',
          '1 when the club has a partner (0x540a50 with the flag) and is its B team'),
         ('pessoas_fora_do_elenco', 'club_collect_non_playing_staff', '0x544c70', 'Database.cpp', 'cdecl',
          'as pessoas fora do elenco (6 de +0xbf, 15 de +0x19f) do clube e do time B num vetor, até o máximo; quantas em `al`',
          'the non-playing staff (6 from +0xbf, 15 from +0x19f) of the club and its B team into an array, up to the maximum; the count in `al`')])
CAMPOS = [('TClub +0xbf..+0xd3 (6) e +0x19f..+0x1d7 (15): cargos fora do elenco (técnico, auxiliar, treinadores, olheiros, fisioterapeutas +0x1cf); ponteiros de pessoa; lê (as três)',
           'TClub +0xbf..+0xd3 (6) and +0x19f..+0x1d7 (15): non-playing jobs (manager, assistant, coaches, scouts, physios +0x1cf); person pointers; reads (all three)'),
          ('TClub +0x244: marca do time B ativo; byte; zerada pela 0x540a50 quando o parceiro é recusado',
           'TClub +0x244: active B team flag; byte; cleared by 0x540a50 when the partner is refused')]
INTERESSA = ['SGE', 'CMRTE']
