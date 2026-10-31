# A tabela da liga (league.cpp): League::add_result (0x685d30, 2.315 chamadas na cobertura de 17/10: cada jogo de liga
# da carreira automática soma na tabela por ela), a cópia da tabela (0x687710, 6.080 chamadas: as projeções da tabela
# copiam, preenchem e ordenam) e as posições previstas (0x68a850, 4 chamadas, passageira). A 0x685d30 chama a 0x685f60
# (já em Rust), a 0x49f570 (estatística da competição, original) e a virtual +4 da fase. A 0x687710 e a 0x68a850 só
# alocam e chamam rotinas já em Rust. Sem SEH; as caixas de erro pela original ou pelo aviso_em. Iguais no
# cm0102-1080.exe.
# Comparadores: a 0x685d30 em ensaio (a cópia sem a 0x49f570) lado a lado com a original (data em +0xc9, linhas e
# confrontos da tabela, +0x3d/+0x3e da partida, devolução); a 0x687710 só pela original (aloca), com a cópia conferida
# contra a tabela inteira; a 0x68a850 só pela original (cria registros), com as posições planejadas antes.
SAVE = False
ROTINAS = [('somar_resultado_na_liga', 0x685d30, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('copiar_tabela_da_liga', 0x687710, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('posicoes_previstas_da_liga', 0x68a850, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela da liga: o resultado na tabela, a cópia da tabela e as posições previstas',
        'the league table: adding the result to the table, copying the table and the predicted positions',
        [('somar_resultado_na_liga', 'League::add_result', '0x685d30', 'league.cpp', 'thiscall, ret 8',
          'grava a data do dia na liga; partida de uma fase vai para a fase; senão soma a partida na tabela (0x685f60) e na estatística da competição (0x49f570) e devolve 1',
          "stamps today's date on the league; a stage fixture goes to its stage; otherwise adds the fixture to the table (0x685f60) and to the competition stats (0x49f570) and returns 1"),
         ('copiar_tabela_da_liga', 'League::copy_table', '0x687710', 'league.cpp', 'thiscall, ret 0',
          'aloca e devolve uma cópia das linhas da tabela, com vetores novos de confrontos e de forma; acerta o +0xa7 quando é −1',
          'allocates and returns a copy of the table rows, with new head-to-head and form buffers; sets +0xa7 when it is -1'),
         ('posicoes_previstas_da_liga', 'League::set_predicted_positions', '0x68a850', 'league.cpp', 'thiscall, ret 0',
          'ordena os clubes da tabela pela força e grava a posição de cada um na estatística da competição',
          "sorts the table's clubs by strength and stores each one's position in the competition stats")])
CAMPOS = [('liga +0xc9: a data da última atualização; valor (data de 8 bytes); grava (somar_resultado_na_liga)',
           'league +0xc9: the last update date; value (8-byte date); writes (somar_resultado_na_liga)'),
          ('liga +0x44: a fase da liga (0xff com fases, 0xfe sem estatística); valor; lê (somar_resultado_na_liga)',
           'league +0x44: the league stage (0xff with stages, 0xfe without stats); value; reads (somar_resultado_na_liga)'),
          ('liga +0xc / +0x30: as fases e a última; ponteiro e valor; lê (somar_resultado_na_liga)',
           'league +0xc / +0x30: the stages and the last one; pointer and value; reads (somar_resultado_na_liga)'),
          ('liga +0xb1 / +0x3e: a tabela (linhas de 0x3b bytes: +0 clube, +5 confrontos, +9 forma) e quantas; ponteiro e valor; lê (as três)',
           'league +0xb1 / +0x3e: the table (0x3b-byte rows: +0 club, +5 head to head, +9 form) and the count; pointer and value; reads (all three)'),
          ('liga +0xa7: o tamanho da forma (−1 = calcular: (linhas − 1) × +0x3c); valor; lê e grava (copiar_tabela_da_liga)',
           'league +0xa7: the form size (-1 = compute: (rows - 1) x +0x3c); value; reads and writes (copiar_tabela_da_liga)'),
          ('liga +8: a estatística da competição; ponteiro; lê (posicoes_previstas_da_liga); registro do clube +0x72: a posição; valor; grava (pela 0x4a2540)',
           "league +8: the competition stats; pointer; reads (posicoes_previstas_da_liga); club entry +0x72: the position; value; writes (via 0x4a2540)"),
          ('partida +0x42: a fase da partida; valor; lê (somar_resultado_na_liga)',
           "fixture +0x42: the fixture's stage; value; reads (somar_resultado_na_liga)"),
          ('[0xadadfc] a tabela das competições; ponteiro; lê (somar_resultado_na_liga)',
           '[0xadadfc] the competition table; pointer; reads (somar_resultado_na_liga)')]
INTERESSA = ['CMRTE']
