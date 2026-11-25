# O gerente de contratos lido do save (CONTRACT_MANAGER::load_from_disk 0x4db5c0, contract_manager.cpp, 3.552 bytes;
# leitor rx07, rodada 6, 25/11/2026). Item de carga (SAVE + CARGA): chamada só pelo construtor 0x4c7040 (rx07_3)
# quando há save (a carga do jogo, LoadStep 0x24). Vai de 0x4db5c0 ao ret 4 de 0x4dc399 (enchimento até 0x4dc3a0).
# Igual no cm0102-1080.exe.
# Convenções conferidas na desmontagem: thiscall(this, save) ret 4 -> 1 lido, 0 falhou; quadro SEH (só para os locais
# STAFF_CONTRACT 0x85e410 e Date 0x54a110 de cada lista de propostas, que nada lê, e o operator new da lista);
# prólogo push -1; push 0x95707c (7), sem relativas. zopen 0x9326e0 cdecl(save, "contract.dat", 0, 0x16, 4); zread
# 0x932c30 cdecl(buf, tamanho, n, z) -> n lidos; malloc 0x944e46; free 0x9452ca; operator new 0x944cf1 (0xe) +
# LLIST::LLIST 0x68d020; LLIST::add_tail 0x68d0d0 thiscall(lista, dados, 0x6f) ret 8; get_next 0x68d1b0; Date::Date
# 0x549f70 thiscall(1, 5, ano, -1) ret 0x10; Date::operator= 0x417c10; zclose 0x932b00 cdecl(&z). As caixas: "Load
# Game Error" 0x9af3e0 com o corpo 0x9af334 e "Memory Allocation Error" 0x9a8108 com 0x9a7ff8 (award_load_error_box).
# Versões (G_GAME_DATA_VERSION pelo valor absoluto): acima de 0x3e000 há as conversas; até 0x3e032 os contratos estão
# no leiaute antigo de 76 bytes; sem o byte de versão no fim (saves antigos), as propostas do "Shevchenko" e as que
# acabam antes de 2000 ganham novas datas.
# Defeitos do original: depois de uma falha o arquivo fica aberto (sem zclose); nos saves até 0x3e032 os contratos
# antigos são lidos para o vetor novo e convertidos a partir do vetor antigo, que nunca foi lido (lixo do malloc); sem
# memória para a primeira lista de propostas, o add_tail roda numa lista nula (cai). A cópia faz o mesmo.
# Comparador: só a original (lê o arquivo uma vez); depois as contagens cabem nas capacidades e cada contrato principal
# leva o id da pessoa. A prova: a carga do FeriasTesteSave com e sem as rotinas (a impressão tem de bater).
SAVE = True
CARGA = True
ROTINAS = [('contract_manager_load_from_disk', 0x4db5c0, [0x6a, 0xff, 0x68, 0x7c, 0x70, 0x95, 0x00])]
_F = 'contract_manager.cpp'
DOCS = ('o gerente de contratos lido do save',
        'the contract manager read from the saved game',
        [('contract_manager_load_from_disk', 'CONTRACT_MANAGER::load_from_disk', '0x4db5c0', _F, 'thiscall, ret 4',
          'lê o contract.dat na ordem em que é gravado: as conversas (saves acima de 0x3e000), os contratos (até 0x3e032 no leiaute antigo de 76 bytes, convertido; depois cada contrato principal volta a levar o id da sua pessoa), as datas de decisão e o índice da próxima, as duas listas de propostas e o byte de versão (sem ele, nos saves antigos, as propostas de quem tem o sobrenome "Shevchenko" acabam em 1º de junho de 2004 e as que acabavam antes de 2000, em 1º de junho de 2002). Fecha o arquivo e confere os empréstimos: um contrato de empréstimo acaba no máximo junto com o principal, e um empréstimo sem contrato principal é desfeito (a pessoa sai de todos os elencos). Cada falha mostra a sua caixa e devolve 0; senão, 1',
          'reads contract.dat in the order it is written: the talks (saves above 0x3e000), the contracts (up to 0x3e032 in the old 76-byte layout, converted; then each main contract names its person again), the decision dates and the next index, the two offer lists and the version byte (without it, in old saves, the offers of anyone with the second name "Shevchenko" end on 1 June 2004 and those ending before 2000 on 1 June 2002). Closes the file and checks the loans: a loan contract ends no later than the main one, and a loan without a main contract is undone (the person leaves every squad). Each failure shows its box and returns 0; otherwise 1')])
CAMPOS = []
INTERESSA = []
