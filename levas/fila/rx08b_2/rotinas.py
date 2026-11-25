# A lista do historico de recordes do clube e a copia de um conjunto de recordes (club_records.cpp; leitor rx08b,
# rodada 6, 25/11/2026). TELAS: a lista e montada so pelas telas do clube (club_screens 0x46c030, 0x46c510 e as
# vizinhas 0x46ccd1, 0x46d185, 0x46d67d, 0x46db35); a copia so pela lista (0x45113e, 0x4511cc). A cobertura do mundo
# viu 0 chamadas de cada.
# Caminho no jogo: Clube -> Recordes (Club -> Records), o seletor de temporadas: "Modern Day", a temporada atual e as
# temporadas arquivadas.
# - 0x450d50 Club_Records::build_history_list thiscall(clube, &n) ret 8, sem SEH: o vetor de ate 500 entradas de 0x38d
#   bytes (malloc 0x6ef64; quem chama libera), a entrada 0 copiada com os ajudantes do operator= do compilador (o
#   0x4ab5b0, 0x4484d0, 0x451570, 0x448230, 0x448500, 0x448530, 0x4485b0, todos ja Rust), as outras pela 0x451200.
# - 0x451200 season_history_entry_assign thiscall(origem) ret 4: a mesma copia de 0x38b bytes, com os recordes
#   partidos (record_holder_assign 0x448490, match_record_assign 0x448110, run_record_assign 0x448170,
#   player_age_record_assign 0x4a1fb0, transfer_fee_record_assign 0x448550); devolve this.
# Prologos: sub esp, 0x208 (6); push ebx; push esi; push edi; mov edi, [esp+0x10] (7). Sem relativo. Iguais no
# cm0102-1080.exe (inteiras).
# Comparadores: a copia lado a lado nos 0x38b bytes; a lista monta o proprio vetor e compara as n entradas com as da
# original (o vetor da copia e liberado).
TELAS = True
SAVE = False
ROTINAS = [('build_history_list', 0x450d50, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('season_history_entry_assign', 0x451200, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10])]
DOCS = ('o histórico de recordes do clube para a tela', "the club's record history for the screen",
        [('build_history_list', 'Club_Records::build_history_list', '0x450d50', 'club_records.cpp', 'thiscall, ret 8',
          'regra: o histórico de recordes do clube para a tela: até 500 entradas de um ano e um conjunto de recordes: primeiro os de sempre (ano 0), depois os da temporada (o ano dela), depois cada temporada arquivada, da mais recente para trás; um conjunto que falta vem zerado como "nenhum"',
          "rule: the club's record history for the screen: up to 500 entries of a year and a record set: first the all-time records (year 0), then this season's (its year), then each archived season from the most recent back; a missing set comes reset to 'none'"),
         ('season_history_entry_assign', 'season_history_entry_assign', '0x451200', 'club_records.cpp', 'thiscall, ret 4',
          'regra: um conjunto de recordes de uma temporada copiado inteiro, campo a campo',
          "rule: a season's club record set copied whole, field by field")])
CAMPOS = []
INTERESSA = []
