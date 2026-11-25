# O construtor do gerente de contratos (CONTRACT_MANAGER::CONTRACT_MANAGER 0x4c7040, contract_manager.cpp, 1.200
# bytes; leitor rx07, rodada 6, 25/11/2026). Simulação, carreira ENG (sem LIGAS), no molde do sbI_2: roda na criação
# do jogo (create_game, CreateStep 0x20 com 0) e na carga (load_game_data, LoadStep 0x24 com o caminho do save, que
# chama o load_from_disk 0x4db5c0 pelo endereço: sem dependência). A rotina vai de 0x4c7040 ao ret 4 de 0x4c74e3.
# O CD pelo find_game_cd (sx02b_1, no HEAD): o patch sem CD do cm0102-1080 e do portátil troca o jne 0x4c7097 por
# nop nop e o test 0x4c70d9 por xor; são os únicos bytes diferentes no 1080 (a rotina é desviada nos dois exes pelos
# dois hashes).
# Convenções conferidas na desmontagem: thiscall(this, save) ret 4 -> this, quadro SEH (só as Dates temporárias);
# prólogo mov eax, fs:[0] (6), sem relativas. 0x54c5b0 Date::operator+ thiscall(G_CURRENT_DATE, &tmp, dias) ret 8;
# 0x417c10 Date::operator= thiscall(global, &tmp, src) ret 8; a procura do CD: GetLogicalDriveStringsA(200),
# GetDriveTypeA = 5, sprintf 0x987e00 "cm0102.exe", 0x4c0aa0/0x4c0c10, a caixa "Insert CD" 0x5e8290 com 1.
# Comparador: só a original (espera o CD e pode ler o save); depois as 16 datas = hoje + os dias da tabela e, sem save,
# os 15 campos zerados.
SAVE = False
ROTINAS = [('new_contract_manager', 0x4c7040, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
_F = 'contract_manager.cpp'
DOCS = ('o construtor do gerente de contratos',
        'the contract manager constructor',
        [('new_contract_manager', 'CONTRACT_MANAGER::CONTRACT_MANAGER', '0x4c7040', _F, 'thiscall, ret 4',
          'monta o gerente de contratos do jogo novo ou carregado: espera o CD do jogo, zera os 15 campos (os vetores de contratos, de datas de decisão e de conversas com as contagens, e as duas listas de propostas) e põe as 16 datas fixas dos contratos em hoje mais 7, 30, 60, 91, 121, 152, 182, 365, 550, 1095, 7, 30, 42, 60, 70 e 91 dias; com um save, lê o contract.dat',
          "builds the contract manager of a new or loaded game: waits for the game CD, clears the 15 fields (the contract, decision date and talks arrays with their counts, and the two offer lists) and sets the 16 fixed contract dates to today plus 7, 30, 60, 91, 121, 152, 182, 365, 550, 1095, 7, 30, 42, 60, 70 and 91 days; with a save, reads contract.dat")])
CAMPOS = []
INTERESSA = []
