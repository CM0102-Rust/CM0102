# Mutações (rx07_4), pela prova do save (o .qst gravado pela cópia contra o da original):
# - o byte de versão sai 0 (a carga só confere se o byte existe): o contract.dat gravado fica diferente;
# - as datas de decisão não são ordenadas antes de gravar.
MUTACOES = [
    ('rx07_4_versao_zero',
     'pub(crate) const CONTRACT_DAT_VERSION: u8 = 1;',
     'pub(crate) const CONTRACT_DAT_VERSION: u8 = 0; // mutation',
     'rx07_4 contract.dat saved'),
    ('rx07_4_sem_ordenar',
     '    if read_u32(this + contract_manager_fields::DECISIONS_ADDED) != 0 {',
     '    if read_u32(this + contract_manager_fields::DECISIONS_ADDED) == 0xdead { // mutation',
     'rx07_4 contract.dat saved'),
]
