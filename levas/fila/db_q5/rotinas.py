ROTINAS = [('clube_basco', 0x53d7f0, [0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9]),
           ('pessoa_listada_pelo_clube', 0x5444a0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('pessoa_listada_para_emprestimo', 0x544590, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('se o clube é basco e se a pessoa está na lista de transferências do clube ou na de empréstimos',
        'whether the club is Basque and whether the person is on the club transfer list or the loan list',
        [('clube_basco', 'database_is_basque_club', '0x53d7f0', 'Database.cpp', 'cdecl',
          'se o clube é da Espanha e um dos 23 clubes bascos citados no código',
          'whether the club is Spanish and one of the 23 Basque clubs named in the code'),
         ('pessoa_listada_pelo_clube', 'staff_is_transfer_listed', '0x5444a0', 'Database.cpp', 'cdecl',
          'se o vínculo da pessoa com o clube (ou o do contrato) está na lista de transferências pelo clube; função 0xf em clube sem técnico humano não conta',
          "whether the person's link with the club (or the contract club) is transfer listed by the club; job 0xf at a club without a human manager does not count"),
         ('pessoa_listada_para_emprestimo', 'staff_is_loan_listed', '0x544590', 'Database.cpp', 'cdecl',
          'se o vínculo da pessoa com o clube (ou o do contrato) está na lista de empréstimos',
          "whether the person's link with the club (or the contract club) is loan listed")])
CAMPOS = [('clube +0x00: id; id; lê (clube_basco: comparado com 23 clubes de [0x9d0120]..[0x9d0338])', 'club +0x00: id; id; reads (clube_basco: compared with 23 clubs at [0x9d0120]..[0x9d0338])'),
          ('clube +0x53: nação; ponteiro; lê (clube_basco)', 'club +0x53: nation; pointer; reads (clube_basco)'),
          ('nação +0x00: id; id; lê (clube_basco: comparado com [0x9cf488] Espanha)', 'nation +0x00: id; id; reads (clube_basco: compared with [0x9cf488] Spain)'),
          ('pessoa +0x3d: função no clube (sem nome em CAMPOS.md; 0xf conta só em clube de técnico humano); valor; lê (pessoa_listada_pelo_clube)', 'staff +0x3d: job at the club (unnamed in CAMPOS.md; 0xf counts only at a human manager club); value; reads (pessoa_listada_pelo_clube)'),
          ('contrato (0x4cf5a0) +4: id do clube; id; lê (pessoa_listada_pelo_clube, pessoa_listada_para_emprestimo)', 'contract (0x4cf5a0) +4: club id; id; reads (pessoa_listada_pelo_clube, pessoa_listada_para_emprestimo)'),
          ('vínculo pessoa-clube (0x4cf5f0) +0x4e: marcas (1 lista de transferências, 2 lista de empréstimos, 8 a pedido); valor; lê (pessoa_listada_pelo_clube: 1; pessoa_listada_para_emprestimo: 2)', 'staff-club link (0x4cf5f0) +0x4e: flags (1 transfer list, 2 loan list, 8 at request); value; reads (pessoa_listada_pelo_clube: 1; pessoa_listada_para_emprestimo: 2)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava (pessoa_listada_pelo_clube, pessoa_listada_para_emprestimo, sem clube)', '[0xae1080] last looked-up contract; pointer; writes (pessoa_listada_pelo_clube, pessoa_listada_para_emprestimo, without a club)')]
INTERESSA = ['CMScout', 'CMRTE']
