# O regen no clube (player_regen_sign_for_club 0x7a6f10, pela PLAYER_REGEN::regenerate_as_youth_player 0x7abac0 em
# todo regen jovem: a vaga no elenco, o valor, o contrato e o fim dele), a aposentadoria pela lesao
# (PLAYER_REGEN::schedule_injury_retirement 0x7a7f20, pela lesao nova 0x62d250 que encerra a carreira) e a noticia da
# aposentadoria anunciada (player_regen_post_retirement_announcement_news 0x7aa340, pela process_retirements 0x7a59e0).
# As tres tem quadro SEH so das datas locais ou da noticia local (sem destrutor que faca algo). Chamadas originais pelo
# endereco: 0x4d0c60 (valor), 0x547d90, 0x4cc0f0, 0x85add0 e 0x85a470 (numeros do elenco), 0x8d1da0 (clubes
# interessados); o resto ja em Rust. Iguais no cm0102-1080.exe. Trocas: das candidatas da familia, a
# process_regen_queue 0x7a65d0 (2,1 KB), a regenerate_as_non_player 0x7ab340 (1,9 KB) e a noticia da aposentadoria
# 0x7a92e0 ficam para outra leva; as de arquivo (0x7ad7c0, 0x7adcd0) e o construtor ficam de fora. Simulacao: as
# mutacoes pela carreira ate 2002-06-01, todas no regen jovem (0x7a6f10: centenas por carreira, cpn_4 "clube melhor
# para o jovem" 512). Comparadores: o regen so a original (o contrato aloca), conferindo o plano (vaga, clube da pessoa,
# devolucao, +0x3a do contrato); a aposentadoria lado a lado na tabela e no sorteio (tabela cheia: so a original); a
# noticia so a original.
SAVE = False
ROTINAS = [('contratar_o_regen', 0x7a6f10, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('aposentar_pela_lesao', 0x7a7f20, [0x6a, 0xff, 0x68, 0xf0, 0xf1, 0x95, 0x00]),
           ('noticia_da_aposentadoria_anunciada', 0x7aa340, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o regen no clube: a vaga no elenco, o valor e o contrato do jovem; a aposentadoria pela lesão e a notícia da aposentadoria anunciada',
        "the regen at the club: the squad slot, the value and the youngster's contract; the retirement through injury and the news of the announced retirement",
        [('contratar_o_regen', 'player_regen_sign_for_club', '0x7a6f10', 'player_regen.cpp', 'stdcall, ret 8',
          'põe o regen na primeira vaga livre das 50 do elenco, com o valor (0x4d0c60; até 5000 no clube de +0x64 = 3), o fim do contrato (abaixo de 19 anos, daqui a dois anos; senão a sorteio(6) + 1) no mês do fim da temporada e dia sorteado, o contrato, o salário padrão e, fora do humano, o fim até daqui a cinco anos; devolve 1 (0 sem vaga)',
          "puts the regen in the first free slot of the 50 in the squad, with the value (0x4d0c60; at most 5000 at a club with +0x64 = 3), the contract end (below 19 years, two years from now; otherwise random(6) + 1) in the club's season end month on a random day, the contract, the default wage and, outside the human club, an end at most five years ahead; returns 1 (0 without a slot)"),
         ('aposentar_pela_lesao', 'PLAYER_REGEN::schedule_injury_retirement', '0x7a7f20', 'player_regen.cpp', 'thiscall, ret 8',
          'tira a aposentadoria pendente da pessoa e acrescenta a nova, pela lesão (motivo 8), daqui a sorteio(dias / 2) + 1 dias',
          "removes the person's pending retirement and adds the new one, through injury (reason 8), random(days / 2) + 1 days ahead"),
         ('noticia_da_aposentadoria_anunciada', 'player_regen_post_retirement_announcement_news', '0x7aa340', 'player_regen.cpp', 'stdcall, ret 8',
          'monta a notícia 0xbbb (id, nomes, motivo, clube do contrato, idade) e manda aos técnicos dos clubes interessados',
          'builds news 0xbbb (id, names, reason, contract club, age) and sends it to the managers of the interested clubs')])
CAMPOS = [('clube +0xd7: as 50 vagas do elenco (ponteiro de pessoa, 0 livre); ponteiro; lê e grava (contratar_o_regen)',
           'club +0xd7: the 50 squad slots (person pointer, 0 free); pointer; reads and writes (contratar_o_regen)'),
          ('clube +0x64: estatuto (3: valor do regen até 5000); valor; lê (contratar_o_regen)',
           'club +0x64: status (3: regen value at most 5000); value; reads (contratar_o_regen)'),
          ('pessoa +0x39: clube; +0x4e: 0; +0x52: valor; +0x46: fim do contrato (data de 8 bytes); +0x18: idade; ponteiro e valor; grava, lê a idade (contratar_o_regen)',
           'person +0x39: club; +0x4e: 0; +0x52: value; +0x46: contract end (8-byte date); +0x18: age; pointer and value; writes, reads the age (contratar_o_regen)'),
          ('contrato +0x2d/+0x2f: fim (dia e ano, até o ano + 5 fora do humano); +0x3a: 0; +0x35 e +0x4f: bits do clube humano; valor; grava (contratar_o_regen)',
           'contract +0x2d/+0x2f: end (day and year, at most year + 5 outside the human club); +0x3a: 0; +0x35 and +0x4f: human club bits; value; writes (contratar_o_regen)'),
          ('PLAYER_REGEN +4: aposentadorias pendentes (26 bytes: +0 id, +4 clube, +8 e +0x10 data, +0x18 motivo 8, +0x19 0), +8 conta, +0xc capacidade; id e valor; tira e acrescenta (aposentar_pela_lesao)',
           'PLAYER_REGEN +4: pending retirements (26 bytes: +0 id, +4 club, +8 and +0x10 date, +0x18 reason 8, +0x19 0), +8 count, +0xc capacity; id and value; removes and appends (aposentar_pela_lesao)'),
          ('[0xae1080] o último contrato lido; ponteiro; grava (noticia_da_aposentadoria_anunciada)',
           '[0xae1080] the last contract looked up; pointer; writes (noticia_da_aposentadoria_anunciada)')]
INTERESSA = ['CMRTE', 'GPF2']
