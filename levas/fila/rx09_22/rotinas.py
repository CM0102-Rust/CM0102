# A fase de liga gravada no jogo salvo (league_stage.cpp; leitor rx09, rodada 6, 25/11/2026). Item do SAVE, no molde do
# cx07_10 (que chama esta pelo endereco: League::save_to_temp_file grava cada fase de liga pela virtual).
# - 0x68ca40 League_Stage::save_to_file thiscall(arquivo) ret 4: sem +0xba ou +0xb1, a caixa 0x259 e nada; as escritas
#   (fwrite 0x945ce6, cdecl, sem conferir) na ordem da carga (0x68b720); a competicao pelo seu primeiro dword ou, sem
#   ela, a caixa 0x276 e -1; as datas (0x41 x +0xa9); +0x18, +0x34 e o vetor de 0x13 bytes; sem clubes, fim; os ponteiros
#   das linhas (e dos adversarios, pela marca da linha 0) viram ids (-1 nenhum); as linhas; a marca do confronto direto
#   (1 com a linha 0) e os vetores de 0xe x (n - 1); a marca do historico (1 com a linha 0; +0xa7 = (n - 1) x +0x3c
#   quando -1) e os historicos de +0xa7 bytes; os ids de volta a ponteiros. Prologo sub esp, 0x218 (6 bytes); igual no
#   cm0102-1080.exe.
# Comparador: so a original (a prova e a do save). Depende do rx09_21 (LEAGUE_STAGE_SOURCE, LEAGUE_STAGE_SAVED_HEAD,
# LEAGUE_STAGE_SAVED_RULES).
SAVE = True
ROTINAS = [('league_stage_save_to_file', 0x68ca40, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00])]
DOCS = ('a fase de liga gravada no jogo salvo', 'the league stage written to the saved game',
        [('league_stage_save_to_file', 'League_Stage::save_to_file', '0x68ca40', 'league_stage.cpp', 'thiscall, ret 4',
          'grava uma fase de liga no save campo a campo, na ordem da carga: as regras, o id da competição, as datas, as linhas da tabela com os clubes como ids, os confrontos diretos e o histórico de posições; depois os clubes voltam',
          'writes a league stage into the save field by field, in the loader\'s order: the rules, the competition id, the dates, the table rows with the clubs as ids, the head-to-head entries and the position history; then the clubs come back')])
CAMPOS = []
INTERESSA = []
