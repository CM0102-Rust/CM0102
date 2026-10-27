# Bloco novo: competições (Cup.cpp). A copa, pelo desfecho de cada time e no fim da temporada: os registros das
# pessoas pelo desfecho (0x519890, posição 0x44 da vtable da copa e 0x902b90; 780 chamadas na carreira), as honras da
# temporada (0x519a90, posição 0x30; 11) e os semifinalistas para a UEFA (0x519fc0, só pela 0x519a90; 2). Simulação: as
# mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem caixa (a 0x53b3d0 e a
# 0x5a0590 só têm caixa no erro), sem sorteio, sem variável local lida sem valor (as saídas da 0x5fa730 só são lidas
# quando ela as grava). As três quase só despacham para originais com efeito que alocam (registros das pessoas
# 0x86a800/0x86b0b0/0x86b710/0x86bdd0/0x86c000, histórico 0x4afce0, semeadura da UEFA 0x905790): o comparador as
# escuta (os `call` da original trocados por porteiros só durante a chamada; a cópia confere contra a fita sem
# executar). Este item traz a escuta (cpb1_*), usada também por cpb_2 e cpb_3 (que vêm depois na fila e saem junto).
# Iguais no cm0102-1080.exe (prólogo e corpo).
SAVE = False
ROTINAS = [('copa_registros_pelo_desfecho', 0x519890, [0x53, 0x56, 0x8b, 0xf1, 0x57]),
           ('copa_honras_da_temporada', 0x519a90, [0x83, 0xec, 0x14, 0x53, 0x55]),
           ('copa_semifinalistas_para_a_uefa', 0x519fc0, [0x83, 0xec, 0x10, 0x53, 0x55])]
DOCS = ('a copa: os registros das pessoas pelo desfecho, as honras da temporada e os semifinalistas para a UEFA',
        'the cup: the staff records by outcome, the season honours and the semi-final losers for UEFA',
        [('copa_registros_pelo_desfecho', 'Cup::update_staff_records_for_status', '0x519890', 'Cup.cpp', 'thiscall',
          'pelo desfecho do clube na copa: registro de campeão (e a vaga na Copa da UEFA pela copa principal da nação ou a da Liga inglesa), vice, terceiro, classificado para a fase seguinte ou eliminado (com a importância da fase)',
          'by the club outcome in the cup: winners record (and the UEFA Cup place through the nation main cup or the English League Cup), runners-up, third place, qualified for the next round or knocked out (with the round importance)'),
         ('copa_honras_da_temporada', 'Cup::record_season_honours', '0x519a90', 'Cup.cpp', 'thiscall',
          'acha o campeão, o vice, o terceiro (na copa galesa, pela estatística) e a seleção anfitriã, grava o histórico e passa os semifinalistas para a UEFA',
          'finds the winner, the runner-up, the third (in the Welsh cup, through the statistics) and the host national team, records the history and passes the semi-final losers to UEFA'),
         ('copa_semifinalistas_para_a_uefa', 'Cup::register_semi_final_losers_for_uefa', '0x519fc0', 'Cup.cpp', 'thiscall',
          'os dois semifinalistas que não estão na final vão para a semeadura da UEFA',
          'the two semi-finalists not in the final go to the UEFA seeding')])
CAMPOS = [('Cup +0x2c, +0x3c (rodadas), +0x3e (times), +0x40 (ano), +0x44 (etapa), +0xa3 (rodadas, 0x68 bytes: +7 tipo, +0x1a confrontos), +0xa7 (times, 6 bytes: clube, +4 nível, +5 desfecho), +0xad (confrontos por rodada, 0x35 bytes); lê (as três)',
           'Cup +0x2c, +0x3c (rounds), +0x3e (teams), +0x40 (year), +0x44 (stage), +0xa3 (rounds, 0x68 bytes: +7 type, +0x1a ties), +0xa7 (teams, 6 bytes: club, +4 level, +5 outcome), +0xad (ties per round, 0x35 bytes); reads (all three)'),
          ('estado do clube [0xae2d14] + id*6: a competição europeia; grava a chave da Copa da UEFA (copa_registros_pelo_desfecho)',
           'club state [0xae2d14] + id*6: the European competition; writes the UEFA Cup key (copa_registros_pelo_desfecho)'),
          ('partida +0x30, +0x32, +0x34 (fase), +0x3a; lê (copa_registros_pelo_desfecho)',
           'fixture +0x30, +0x32, +0x34 (round), +0x3a; reads (copa_registros_pelo_desfecho)')]
INTERESSA = ['SGE', 'CMRTE']
