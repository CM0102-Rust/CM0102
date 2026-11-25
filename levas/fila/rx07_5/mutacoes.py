# Mutações (rx07_5), pela prova da carga (o FeriasTesteSave carregado com e sem as rotinas, a impressão comparada):
# - a próxima decisão a tratar não é lida (fica 0): as decisões já tratadas rodam de novo depois da carga;
# - o contrato de empréstimo não é cortado no fim do contrato principal.
MUTACOES = [
    ('rx07_5_indice_nao_lido',
     '    contract_read(this + contract_manager_fields::DECISION_NEXT_INDEX, 4, 1, z, Read(0x315e, 0x9af4c0))?;',
     '    let mut skipped = 0u32; contract_read(&mut skipped as *mut u32 as usize, 4, 1, z, Read(0x315e, 0x9af4c0))?; // mutation',
     'rx07_5 contract.dat loaded'),
    ('rx07_5_emprestimo_nao_cortado',
     '            if my < ly || (my == ly && read_i16(main + contract::CONTRACT_EXPIRES) < read_i16(loan + contract::CONTRACT_EXPIRES)) {',
     '            if my < ly && my == 0xffff { // mutation',
     'rx07_5 loan end cut to the main contract'),
]
