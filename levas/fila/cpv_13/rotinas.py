# O prêmio do mês no histórico (staff_records.cpp): Staff_Records::add_month_award_record (0x86c8c0, 120 chamadas na
# cobertura de 17/10) e dois ajudantes do amistoso (friendly.cpp), passageiros (não rodam na carreira automática):
# friendly_club_reputation_tier (0x5c52d0) e friendly_get_rest_days_for_kind (0x5c74d0). A 0x86c8c0 tem quadro SEH sem
# destrutor (a entrada local do histórico não tem); usa os ajudantes do histórico e da renovação da 253ª (cpq3_,
# cpq4_: entrada, data, arquivo FILE_LLIST, registro de pontos 0x86f5b0, oferta 0x4cda70). Iguais no cm0102-1080.exe.
# Comparadores: o prêmio do técnico lado a lado pelo registro de pontos dele (a cópia sem o arquivo do histórico, como a
# cpq_3); o do jogador só pela original (sorteio, renovação e notícia); os do amistoso puros (a caixa da folga, só a
# original).
SAVE = False
ROTINAS = [('registrar_premio_do_mes', 0x86c8c0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('faixa_de_reputacao_do_amistoso', 0x5c52d0, [0x8b, 0x44, 0x24, 0x04, 0x66, 0x8b, 0x80, 0x80, 0x00, 0x00, 0x00]),
           ('folga_do_amistoso', 0x5c74d0, [0x0f, 0xbe, 0x44, 0x24, 0x04])]
DOCS = ('o prêmio do mês no histórico (o técnico conta, o jogador agradece ou renova), a faixa de reputação e a folga do amistoso',
        "the monthly award in the history (the manager counts it, the player thanks him or renews), the friendly's reputation tier and rest days",
        [('registrar_premio_do_mes', 'Staff_Records::add_month_award_record', '0x86c8c0', 'staff_records.cpp', 'thiscall, ret 0x14',
          'grava a entrada 0x33 do histórico do premiado; o técnico conta +1 no registro dele; o jogador pode ter a renovação do contrato e, por sorteio, agradece ao técnico (notícia)',
          "writes the 0x33 history entry of the winner; a manager counts +1 in his record; a player may get a contract renewal and, by a draw, thanks his manager (news)"),
         ('faixa_de_reputacao_do_amistoso', 'friendly_club_reputation_tier', '0x5c52d0', 'friendly.cpp', 'stdcall, ret 4',
          'a faixa 1..5 da reputação do clube (2000, 4000, 6000 e 8000)',
          "the club's reputation tier 1..5 (2000, 4000, 6000 and 8000)"),
         ('folga_do_amistoso', 'friendly_get_rest_days_for_kind', '0x5c74d0', 'friendly.cpp', 'stdcall, ret 4',
          'os dias de folga em volta do amistoso pelo tipo: 2 nos tipos 1 e 10, 3 no 2, senão 1',
          'the rest days around a friendly by kind: 2 for kinds 1 and 10, 3 for kind 2, else 1')])
CAMPOS = [('histórico da pessoa (FILE_LLIST [Staff_Records], 0x58e340): a entrada 0x33 com a data, o clube ou −1 (+8), o id do prêmio (+0xd), a4 (+0x11) e a5 (+0x15); valor; grava (registrar_premio_do_mes)',
           "staff history (FILE_LLIST [Staff_Records], 0x58e340): the 0x33 entry with the date, club or −1 (+8), award id (+0xd), a4 (+0x11) and a5 (+0x15); value; writes (registrar_premio_do_mes)"),
          ('staff +0x3d e +0x28: o cargo (5 ou 0xc = técnico); valor; lê (registrar_premio_do_mes)',
           'staff +0x3d and +0x28: the job (5 or 0xc = manager); value; reads (registrar_premio_do_mes)'),
          ('registro de pontos do técnico (0x86f5b0, 0xd2 bytes) +0x9e: os prêmios do mês (palavra); valor; lê e grava +1 (registrar_premio_do_mes)',
           "manager points record (0x86f5b0, 0xd2 bytes) +0x9e: the monthly awards (word); value; reads and writes +1 (registrar_premio_do_mes)"),
          ('staff +0x59 e +0x5b: os bytes que liberam o sorteio do agradecimento (acima de 15 e de 10); +0x59 é o teto do sorteio; valor; lê (registrar_premio_do_mes)',
           'staff +0x59 and +0x5b: the bytes that allow the thanks draw (above 15 and 10); +0x59 is the draw ceiling; value; reads (registrar_premio_do_mes)'),
          ('contrato (0x4cf5f0) +0x25/+0x29: o começo (mais de 300 dias para a renovação); +0x45 a marca que barra o agradecimento; valor; lê (registrar_premio_do_mes)',
           'contract (0x4cf5f0) +0x25/+0x29: the start (over 300 days for the renewal); +0x45 the flag that blocks the thanks; value; reads (registrar_premio_do_mes)'),
          ('[0xae1080] o contrato achado por último; ponteiro; grava (registrar_premio_do_mes)',
           '[0xae1080] the last contract found; pointer; writes (registrar_premio_do_mes)'),
          ('clube +0x80: a reputação; valor; lê (faixa_de_reputacao_do_amistoso)',
           "club +0x80: the reputation; value; reads (faixa_de_reputacao_do_amistoso)")]
INTERESSA = ['CMRTE', 'GPF2']
