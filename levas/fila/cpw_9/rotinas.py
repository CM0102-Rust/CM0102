# O dia das lesoes (Injury::daily_update 0x62b7b0, pelo laco do dia 0x5cccd0, todo dia: os contadores das lesoes que
# envelhecem e as lesoes no treino, [0xae2364] / 150 sorteios de uma pessoa nos dias de semana, com a lesao nova
# 0x62d250 e os atributos que ela gasta), as noticias da volta aos treinos (Injury::post_due_recovery_news 0x62e010,
# pelo dia das lesoes, todo dia: o fisico dos lesionados dos clubes humanos e a fila das voltas) e o jogador a um cartao
# da suspensao (Discipline::is_one_booking_from_ban 0x55d360, pela tela do clube 0x4849f0). Chamadas originais pelo
# endereco: 0x68d040 e 0x944c94 (soltar a lista das batidas), 0x456770/0x456740 (datas), 0x418f60 (ausente), 0x62c680
# (o fisico do dia, cpw_8), 0x62d250 (a lesao nova), 0x630ee0 (a noticia da volta), 0x54cc30, 0x946464 (bsearch),
# 0x7e8d60 (o orgao da competicao); o resto ja em Rust. Iguais no cm0102-1080.exe. Simulacao: as mutacoes pela carreira
# ate 2002-06-01, todas nas lesoes no treino. Comparadores: o dia das lesoes lado a lado na fase do treino (cabecalho do
# Injury, tabela de lesoes e sorteio): a copia em ensaio (sem soltar a lista, sem a lesao nova nem as rotinas do fim) e a
# original conferida quando chega as noticias da volta (0x62e010, desviada no mesmo item); com lesao nova ou com o fisico
# de todos (dia 180 da primeira temporada), so a original; as noticias da volta so a original (alocam); o cartao pela
# devolucao.
SAVE = False
ROTINAS = [('dia_das_lesoes', 0x62b7b0, [0x83, 0xec, 0x1c, 0x53, 0x8b, 0xd9]),
           ('noticias_de_volta_aos_treinos', 0x62e010, [0x6a, 0xff, 0x68, 0x68, 0xb1, 0x95, 0x00]),
           ('um_cartao_da_suspensao', 0x55d360, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('o dia das lesões: as lesões que envelhecem e as lesões no treino; as notícias da volta aos treinos e o jogador a um cartão da suspensão',
        'the injury day: ageing injuries and training injuries; the return-to-training news and the player one booking away from a ban',
        [('dia_das_lesoes', 'Injury::daily_update', '0x62b7b0', 'injury.cpp', 'thiscall, ret 0',
          'solta a lista das batidas da partida, desce os dois contadores de cada lesão, no dia 180 da primeira temporada o físico de todos; nos dias de semana [0xae2364] / 150 sorteios (pelo menos 10) de uma pessoa: o jogador presente e sem lesão que passa pelos sorteios de risco (ficha +0x2d, propensão, recorrência) ganha a categoria, o tipo (0x62d140), o gasto dos atributos e a lesão nova (0x62d250); no fim as notícias da volta (0x62e010) e os surtos (0x62e9e0)',
          'frees the match knock list, lowers both counters of every injury, on day 180 of the first season the fitness of everyone; on weekdays [0xae2364] / 150 draws (at least 10) of a person: a present, uninjured player who passes the risk draws (player data +0x2d, proneness, recurrence) gets the category, the type (0x62d140), the attribute wear and the new injury (0x62d250); at the end the return news (0x62e010) and the outbreaks (0x62e9e0)'),
         ('noticias_de_volta_aos_treinos', 'Injury::post_due_recovery_news', '0x62e010', 'injury.cpp', 'thiscall, ret 0',
          'nos clubes dos técnicos humanos, a notícia da volta ao treino leve (uma vez, +0x1c) e o físico do dia de cada lesionado; na fila das voltas, as de data A hoje e as de data B hoje dão a notícia (0x630ee0) e as de data B passada saem do começo',
          "at the human managers' clubs, the light training return news (once, +0x1c) and the daily fitness of every injured player; in the return queue, the ones with date A today and date B today get the news (0x630ee0) and the ones with past date B leave the front"),
         ('um_cartao_da_suspensao', 'Discipline::is_one_booking_from_ban', '0x55d360', 'discipline.cpp', 'thiscall, ret 8',
          'com id válido, a pergunta ao órgão da competição (+0x50; 0x7e8d60 com o id e o +4 da competição); senão 0',
          "with a valid id, the question to the competition's ruling body (+0x50; 0x7e8d60 with the id and the competition's +4); otherwise 0")])
CAMPOS = [('Injury +0x28: lista das batidas da partida; ponteiro; solta e grava 0 (dia_das_lesoes)',
           'Injury +0x28: match knock list; pointer; frees and writes 0 (dia_das_lesoes)'),
          ('Injury +4: tabela de lesões de 0x30 bytes (+0 tipo, +0x20 começo, +0x28 fim, +0x2a dias parados, +0x2c e +0x2e contadores); +8 conta; valor; lê e grava (dia_das_lesoes, noticias_de_volta_aos_treinos)',
           'Injury +4: 0x30-byte injury table (+0 type, +0x20 start, +0x28 end, +0x2a stopped days, +0x2c and +0x2e counters); +8 count; value; reads and writes (dia_das_lesoes, noticias_de_volta_aos_treinos)'),
          ('Injury +0: registro de 31 bytes (+0xe lesão atual, +0x12 categoria da recorrência, +0x13 peso dela, +0x1c notícia da volta dada, +0x1e propensão); índice e valor; lê, grava o +0x1c (dia_das_lesoes, noticias_de_volta_aos_treinos)',
           'Injury +0: 31-byte record (+0xe current injury, +0x12 recurrence category, +0x13 its weight, +0x1c return news given, +0x1e proneness); index and value; reads, writes +0x1c (dia_das_lesoes, noticias_de_volta_aos_treinos)'),
          ('Injury +0x10: fila das voltas (22 bytes: +0 data A, +8 data B, +0x10 id, +0x14 e +0x15), conta +0x18; valor; lê, tira do começo (noticias_de_volta_aos_treinos)',
           'Injury +0x10: return queue (22 bytes: +0 date A, +8 date B, +0x10 id, +0x14 and +0x15), count +0x18; value; reads, removes from the front (noticias_de_volta_aos_treinos)'),
          ('ficha +0xf nota, +0x2d, +0x1b, +0x1d, +0x36 (gastos pela lesão); pessoa +0x3d cargo; valor; lê e grava (dia_das_lesoes)',
           'player data +0xf rating, +0x2d, +0x1b, +0x1d, +0x36 (worn by the injury); person +0x3d job; value; reads and writes (dia_das_lesoes)'),
          ('[0x9e7ad9] tabela de tipos de lesão (13 bytes: categoria); [0x9e7f05] faixas das categorias (3 bytes); valor; lê (dia_das_lesoes)',
           '[0x9e7ad9] injury type table (13 bytes: category); [0x9e7f05] category ranges (3 bytes); value; reads (dia_das_lesoes)'),
          ('competição +0x50 órgão (0xff sem), +4; Discipline +0 órgãos; ponteiro e valor; lê (um_cartao_da_suspensao)',
           'competition +0x50 ruling body (0xff none), +4; Discipline +0 ruling bodies; pointer and value; reads (um_cartao_da_suspensao)')]
INTERESSA = ['CMRTE']
