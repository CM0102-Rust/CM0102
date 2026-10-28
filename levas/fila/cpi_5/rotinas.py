# Listas e vagas: tirar as ofertas guardadas de uma pessoa (CONTRACT_MANAGER::remove_held_offers 0x4cf560, chamada
# pelas 0x8c3310/0x8c34e0 das transferências; a 0x68d1f0 tira o item), guardar as suspensões que não valem no clube do
# empréstimo (Discipline::hold_bans_for_loan 0x5563f0, pelas 0x8bf400/0x8c2180; a 0x68d0d0 acrescenta; ver
# docs/MAPA_DISCIPLINA.md) e preencher a vaga da comissão que ficou livre (fill_vacated_staff_job 0x8cb6d0, pela
# 0x8c9720; a 0x53bd10 já em Rust; ver docs/COMISSAO_REPETIDA.md). Simulação: as mutações pela carreira e a
# confirmação no jogo real comparando. Sem SEH, sem sorteio; caixa só com ficha ou clube nulos (a original). A
# 0x8cb6d0 troca o byte do cargo no argumento (13 → 6, 15 → 8) e passa o dword inteiro à 0x53bd10 (que só lê o byte);
# a cópia faz o mesmo. Comparadores: a original roda e o resultado é conferido com o plano da cópia (os itens que
# ficaram; o registro de suspensões; a pessoa escolhida agora no clube).
SAVE = False
ROTINAS = [('tirar_ofertas_guardadas', 0x4cf560, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x38]),
           ('guardar_suspensoes_do_emprestado', 0x5563f0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('preencher_vaga_da_comissao', 0x8cb6d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('tirar as ofertas guardadas da pessoa, guardar as suspensões do emprestado e preencher a vaga da comissão',
        'removing the held offers of a person, holding the bans of a loaned player and filling the vacated staff job',
        [('tirar_ofertas_guardadas', 'CONTRACT_MANAGER::remove_held_offers', '0x4cf560', 'contract_manager.cpp', 'thiscall, ret 4',
          'tira da lista das ofertas guardadas (+0x38) cada item da pessoa (0x68d1f0)',
          'removes from the held offers list (+0x38) every item of the person (0x68d1f0)'),
         ('guardar_suspensoes_do_emprestado', 'Discipline::hold_bans_for_loan', '0x5563f0', 'discipline.cpp', 'thiscall, ret 4',
          'as suspensões (vagas 0 a 2) que não valem no clube do empréstimo vão para a lista das guardadas (+0x1c), a vaga fica −1 e o contador +0x1c sobe',
          'the bans (slots 0 to 2) that do not apply at the loan club go to the held list (+0x1c), the slot becomes −1 and the +0x1c counter goes up'),
         ('preencher_vaga_da_comissao', 'fill_vacated_staff_job', '0x8cb6d0', 'transfer_manager.cpp', 'stdcall, ret 8',
          'a primeira pessoa desempregada da nação do clube no cargo procurado (6, 8, 9, 10; 13 → 6, 15 → 8) entra no clube (0x53bd10)',
          'the first unemployed person of the club nation in the wanted job (6, 8, 9, 10; 13 → 6, 15 → 8) joins the club (0x53bd10)')])
CAMPOS = [('gerenciador de contratos +0x38 lista das ofertas guardadas; disciplina [+0] tipos de suspensão (+0x17 competição, +0x1b), +0x14 registros de 29 bytes (vagas de 5 bytes, +0x1c contador), +0x1c lista das guardadas (10 bytes); ficha de contrato +4 clube; nação +0x71; pessoa +0x1a nação, +0x39 clube, +0x3d cargo, +0x69; escreve (tirar_ofertas_guardadas, guardar_suspensoes_do_emprestado, preencher_vaga_da_comissao)',
           'contract manager +0x38 held offers list; discipline [+0] ban types (+0x17 competition, +0x1b), +0x14 records of 29 bytes (5-byte slots, +0x1c counter), +0x1c held list (10 bytes); contract card +4 club; nation +0x71; person +0x1a nation, +0x39 club, +0x3d job, +0x69; writes (tirar_ofertas_guardadas, guardar_suspensoes_do_emprestado, preencher_vaga_da_comissao)')]
INTERESSA = ['SGE', 'CMRTE']
