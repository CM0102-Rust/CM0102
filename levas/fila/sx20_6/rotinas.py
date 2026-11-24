# Montar os registros de pessoas (staff_records.cpp; leitor sx20, rodada 5, 24/11/2026): Staff_Records::Staff_Records
# (0x868390). Rotina inteira (0x868390..0x8687bc ret 4). Quem chama: a criacao do jogo (0x828d04) e a carga do save
# (0x82bd86). Na cobertura ENG roda 1 vez.
# DEPENDE DO sx20_2 (integrar junto ou depois): usa wait_for_game_cd (e o sx20_2 depende do sx20_1).
# Convencoes conferidas na desmontagem:
# - thiscall(this, save) ret 4 -> this; quadro SEH so pelos new dos FILE_LLIST;
# - 0x549f70 Date::Date(13, 11, 1975, -1) num local que ninguem le (a copia nao monta);
# - a busca do CD sem condicao, como a do SimulatedStats (GetLogicalDriveStringsA, GetDriveTypeA 5, "%s" + cm0102.exe,
#   0x4c0aa0 / 0x4c0c10, "Insert CD"); no cm0102-1080.exe o jne de 0x86840b vira nop e o test de 0x86844e vira xor;
# - 0x944cf1 new(8), 0x90bde0 utils_build_file_path cdecl("CM3_TEMP", nome), 0x58e280 FILE_LLIST::FILE_LLIST
#   thiscall(lista, caminho, 0x35 / 0x31) ret 8 (a club_history_new_file_list); 0x945501 calloc(nao jogadores + 16,
#   0xd2); 0x944e46 malloc(pessoas * 8) e malloc(clubes * 8); sem memoria a caixa (staff_records.cpp 0x87, 0x8f, 0x97),
#   [0xb67a34] = 0 e devolve this (nao para o jogo);
# - com save 0x86fce0 Staff_Records::load thiscall(this, save) ret 4 -> eax; 0 refaz do zero.
# Defeitos do original: sem memoria os registros ficam pela metade e o jogo segue (sem conferencia depois).
ROTINAS = [('staff_records_staff_records', 0x868390, [0x6a, 0xff, 0x68, 0x47, 0x2b, 0x96, 0x00])]
DOCS = ('montar os registros de pessoas',
        'building the staff records',
        [('staff_records_staff_records', 'Staff_Records::Staff_Records', '0x868390', 'staff_records.cpp', 'thiscall, ret 4',
          'monta os registros de pessoas: espera o CD, zera o mes e o ano guardados, abre as listas temporarias de historico de pessoas ("staff history.tmp", 0x35 bytes) e de clubes ("club history.tmp", 0x31), aloca os registros de tecnico (nao jogadores + 16, zerados) e as cabecas de lista (8 bytes por pessoa e por clube); sem memoria, a caixa e fica pela metade; com save, carrega; sem save ou se a carga falha, todo registro de tecnico fica sem dono, nacao da temporada, nacao da selecao, clube atual, mais longo e mais curto (-1), cada tecnico de clube ou de selecao (cargo 5 ou 12) com registro de nao jogador fica dono do seu com um cargo de clube ou de selecao, e toda cabeca de lista fica vazia (-1); devolve this',
          'builds the staff records: waits for the CD, clears the stored month and year, opens the person ("staff history.tmp", 0x35 bytes) and club ("club history.tmp", 0x31) history temporary lists, allocates the manager records (non-players + 16, zeroed) and the list heads (8 bytes per person and per club); out of memory, the box and it stays half built; with a save, loads; without one or when the load fails, every manager record has no owner, season nation, national nation, current, longest or shortest club (-1), each club or national team manager (job 5 or 12) with a non-player record owns his with one club or national job, and every list head is empty (-1); returns this')])
