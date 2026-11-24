# Limpar bonus e clausulas de um contrato (staff_contracts.cpp) e a ordem das listas de posicao das listas curtas
# (shortlist_manager.cpp) (leitor sx20, rodada 5, 24/11/2026): STAFF_CONTRACT::clear_bonuses_and_clauses (0x85f800) e
# shortlist_compare_by_value_then_key (0x83b670). Rotinas inteiras (0x85f800..0x85f823 ret; 0x83b670..0x83b68c ret).
# DEPENDE DO sx20_3 (integrar junto ou depois): usa sx20_compare_pure.
# Quem chama: a limpeza, process_retirements (0x7a5c18; em Rust pelo endereco FN_STAFF_CONTRACT_CLEAR_BONUSES_AND_CLAUSES,
# na aposentadoria de quem e jogador e tecnico; 274 chamadas na cobertura do mundo); a ordem, o qsort das listas de
# posicao em create_initial_shortlists (0x83a98c, 0x83a9a5; criacao do jogo).
# Convencoes conferidas na desmontagem: 0x85f800 thiscall(contrato), ret, so escreve (+0x49 = 0, +0x4a = -1, +0x1c..+0x20
# = 0, +0x10/+0x14/+0x18 = -1); 0x83b670 cdecl(a, b) -> int.
ROTINAS = [('staff_contract_clear_bonuses_and_clauses', 0x85f800, [0x32, 0xc0, 0x83, 0xca, 0xff]),
           ('shortlist_compare_by_value_then_key', 0x83b670, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('limpar bonus e clausulas de um contrato e a ordem das listas de posicao',
        "clearing a contract's bonuses and clauses and the order of the position lists",
        [('staff_contract_clear_bonuses_and_clauses', 'STAFF_CONTRACT::clear_bonuses_and_clauses', '0x85f800', 'staff_contracts.cpp', 'thiscall',
          'tira todos os bonus e clausulas de rescisao do contrato: bonus de gol, assistencia e jogo sem sofrer gol = -1, as clausulas de nao promocao, multa minima, +0x1e, rebaixamento e cargo de tecnico = 0, e deixa de sair pelo Bosman e de ter transferencia acertada (-1)',
          'removes every bonus and release clause of the contract: goal, assist and clean sheet bonuses = -1, the non-promotion, minimum fee, +0x1e, relegation and manager job clauses = 0, and it is no longer leaving on a Bosman nor has a transfer arranged (-1)'),
         ('shortlist_compare_by_value_then_key', 'shortlist_compare_by_value_then_key', '0x83b670', 'shortlist_manager.cpp', 'cdecl',
          'a ordem das listas de posicao (entradas de 12 bytes) no qsort: pelo valor em +8, menor primeiro, depois pela chave em +4',
          'the qsort order of the position lists (12-byte entries): by the value at +8, lowest first, then by the key at +4')])
