# As faltas do dia (awol.cpp): AWOL::daily_update 0x4190d0 (todo dia pelo laço do jogo 0x5cccd0: as ausências, as
# faltas ao treino marcadas pela 0x41a360, que a 255ª conta em "cps_6 missed training", e os atrasos),
# AWOL::has_missed_training 0x418ff0 e AWOL::has_fitness_incident 0x419070 (pelos jogadores do dia 0x5a69b0 e pelas
# telas). Família fora das do brief (finance, club, stadium, game e human_manager esgotadas para simulação): awol.cpp,
# que nenhum outro agente usa. Iguais no cm0102-1080.exe; sem SEH; sem x87. Comparadores: a 0x4190d0 só a original
# (sorteia, tira e libera nós das listas, dá notícias e faltas novas; a cópia leva os ramos); a 0x418ff0 e a 0x419070
# lado a lado (a devolução no byte baixo, o [0xae1080] e o cursor da lista), a 0x418ff0 só a original quando tira a
# falta da lista (pessoa com o bit 2 e sem contrato).
SAVE = False
ROTINAS = [('faltas_do_dia', 0x4190d0, [0x83, 0xec, 0x1c, 0x53, 0x55]),
           ('teve_falta_ao_treino', 0x418ff0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('teve_incidente_de_forma', 0x419070, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('marcar_falta_ao_treino', 0x41a360, [0x6a, 0xff, 0x68, 0xe8, 0x3e, 0x95, 0x00]),
           ('fim_da_ausencia', 0x419710, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('marcas_do_contrato', 0x867560, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as faltas do dia: o dia das ausências, das faltas ao treino e dos atrasos, a falta ao treino marcada e o atraso registrado',
        'the daily absences: the day of the AWOL players, the missed training and the late arrivals, the missed training flag and the fitness incident record',
        [('faltas_do_dia', 'AWOL::daily_update', '0x4190d0', 'awol.cpp', 'thiscall',
          'acaba a primeira ausência vencida; resolve a primeira falta ao treino de uma semana atrás (caso repetido: sorteios e nova falta; caso novo: colegas que pedem a saída ou perdem profissionalismo, moral e temperamento, e o faltoso que perde profissionalismo e moral e briga com o clube); resolve o primeiro atraso vencido pelos sorteios',
          'ends the first expired AWOL; resolves the first missed training from a week ago (repeat case: draws and a new miss; new case: teammates who ask to leave or lose professionalism, morale and temperament, and the offender who loses professionalism and morale and falls out with the club); resolves the first expired late arrival by the draws'),
         ('teve_falta_ao_treino', 'AWOL::has_missed_training', '0x418ff0', 'awol.cpp', 'thiscall, ret 4',
          'o bit 2 da marca da pessoa; sem contrato, tira a falta antes',
          'bit 2 of the person flag; without a contract, clears the miss first'),
         ('teve_incidente_de_forma', 'AWOL::has_fitness_incident', '0x419070', 'awol.cpp', 'thiscall, ret 4',
          'verdadeiro se a pessoa tem registro na lista dos atrasos',
          'true when the person has a record in the late arrival list'),
         ('marcar_falta_ao_treino', 'AWOL::miss_training', '0x41a360', 'awol.cpp', 'thiscall, ret 4',
          'o jogador muito infeliz pode sumir (0x419840); senão marca a falta, dá a notícia, cai a condição, entra na lista das faltas (o byte do caso, sem valor na original, vai 0x90) e o técnico da IA pode multar',
          'the very unhappy player may go AWOL (0x419840); otherwise marks the miss, posts the news, lowers the condition, joins the missed training list (the case byte, unset in the original, goes 0x90) and the AI manager may fine him'),
         ('fim_da_ausencia', 'AWOL::end_absence', '0x419710', 'awol.cpp', 'thiscall, ret 4',
          'com contrato num clube, a notícia da volta; troca o bit 1 da marca e tira o registro da lista das ausências',
          'with a contract at a club, the return news; toggles bit 1 of the flag and removes the record from the absence list'),
         ('marcas_do_contrato', 'STAFF_CONTRACT::set_flags', '0x867560', 'staff_contracts.cpp', 'thiscall, ret 4',
          'liga as marcas em +0x40 e, se a pessoa agora quer sair e nada impede, o pedido de transferência (0x4d8610)',
          'sets the flags at +0x40 and, when the person now wants to leave and nothing prevents it, the transfer request (0x4d8610)')])
CAMPOS = [('AWOL +0 ausências, +4 faltas ao treino, +8 atrasos (listas com conta em +0, primeiro nó em +2, cursor em +0xa), +0xc marcas por pessoa (bit 1 ausente, bit 2 faltou); ponteiros/valores; lê e grava (faltas_do_dia, teve_falta_ao_treino, teve_incidente_de_forma)',
           'AWOL +0 absences, +4 missed training, +8 late arrivals (lists with count at +0, first node at +2, cursor at +0xa), +0xc per-person flags (bit 1 AWOL, bit 2 missed training); pointers/values; reads and writes (faltas_do_dia, teve_falta_ao_treino, teve_incidente_de_forma)'),
          ('registro da falta (13 bytes) +0 pessoa, +4 data, +0xc caso repetido; registro do atraso +0 pessoa, +8 data, +0x10 marca; id/valores; lê (faltas_do_dia)',
           'missed training record (13 bytes) +0 person, +4 date, +0xc repeat case; late arrival record +0 person, +8 date, +0x10 flag; id/values; reads (faltas_do_dia)'),
          ('pessoa +0x18 idade, +0x57 atributo, +0x5b profissionalismo, +0x5d temperamento; valores; lê e grava +0x5b e +0x5d (faltas_do_dia)',
           'staff +0x18 age, +0x57 attribute, +0x5b professionalism, +0x5d temperament; values; reads and writes +0x5b and +0x5d (faltas_do_dia)'),
          ('jogador (pessoa +0x61) +0xb reputação atual, +0x44 moral; valores; lê e grava +0x44 (faltas_do_dia)',
           'player (staff +0x61) +0xb current reputation, +0x44 morale; values; reads and writes +0x44 (faltas_do_dia)'),
          ('clube +0xd7 elenco (50 vagas); contrato +0x4f tipo (nibble alto); [0xae1080] último contrato; ponteiros/valores; lê, grava [0xae1080] (faltas_do_dia, teve_falta_ao_treino)',
           'club +0xd7 squad (50 slots); contract +0x4f type (high nibble); [0xae1080] last contract; pointers/values; reads, writes [0xae1080] (faltas_do_dia, teve_falta_ao_treino)')]
CAMPOS += [('contrato +0x40 marcas, +0x44, +0x45 bits, +0x49, +0x4e bits 1 e 8; valores; lê, grava +0x40 (marcas_do_contrato, marcar_falta_ao_treino)',
            'contract +0x40 flags, +0x44, +0x45 bits, +0x49, +0x4e bits 1 and 8; values; reads, writes +0x40 (marcas_do_contrato, marcar_falta_ao_treino)'),
           ('[0xae2d0c] linhas de 79 bytes por pessoa, +0xa; valor; soma sorteio(10) (marcar_falta_ao_treino); pessoa +0x3d função, +0x59 atributo; lê',
            '[0xae2d0c] 79-byte rows per person, +0xa; value; adds a draw of 10 (marcar_falta_ao_treino); staff +0x3d job, +0x59 attribute; reads')]
INTERESSA = ['CMRTE']
