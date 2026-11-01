# Mutacoes (cpx_5) pela prova do save (mutacoes.py --prova-save N): o .qst gravado no modo rust com a mutacao contra
# a base (a mesma DLL sem as rotinas da leva). Sem o 4o campo (contador): a base da prova do save roda com as rotinas
# desligadas, entao os ramos nao contariam; a prova e o item do .qst.
# - os registros das financas com 0x166 bytes em vez de 0x167: o finance.dat sai 2.520 bytes menor (2.520 clubes no .qst da prova) e deslocado
#   (o zwrite devolve a quantidade de registros, sem caixa);
# - o gerente de elencos gravado com +1: o squad_manager.dat sai 01 00 00 00 (o jogo novo grava 0);
# - a carga das financas devolvendo 0: o construtor (0x59a55c) refaz as financas de cada clube do zero (0x5961d0) por
#   cima das lidas; so a carga do FeriasTesteSave (CARGA) acusa, pela impressao ate 2001-08-28 (saldos refeitos mudam
#   as decisoes de transferencia e os sorteios). Pode escapar se nenhum sorteio ate 2001-08-28 depender do saldo.
MUTACOES = [
    ('cpx5_registro_curto', '    if gravar_item(cpx5_d(this), 0x167, cpx5_clubes(), z) != cpx5_clubes() { // cpx5: os registros das finanças',
     '    if gravar_item(cpx5_d(this), 0x166, cpx5_clubes(), z) != cpx5_clubes() { // mutacao: registro curto'),
    ('cpx5_gerente_mais_um', '    let mut v = *(this as *const u16) as u32; // cpx5: o valor do gerente de elencos',
     '    let mut v = *(this as *const u16) as u32 + 1; // mutacao: o valor mais um'),
    ('cpx5_carga_refeita', '    1 // cpx5: carregou (o construtor não refaz as finanças)',
     '    0 // mutacao: o construtor refaz as financas'),
]
