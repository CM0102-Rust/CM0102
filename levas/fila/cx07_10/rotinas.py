# O arquivo temporario de uma liga (league.cpp; leitor cx07, rodada 5, 24/11/2026). Item do SAVE (gravacao): roda a cada
# save, para toda liga ativa (slot 0x1c da League, pelo save_game). A carga (League::load_from_temp_file 0x687b10) ja e
# Rust (gra_29): a ordem dos campos e a mesma.
# - 0x6891c0 League::save_to_temp_file thiscall(por_id) ret 4 (2320 bytes, sozinha no item): sprintf 0x944c9f (cdecl,
#   "%s" e "%s_%d.tmp"), utils_build_file_path 0x90bde0 e fopen 0x945b00 (cdecl, "wb"), fwrite 0x945ce6 (cdecl, 4
#   pushes), Comp_Stats::save 0x4ac4e0 (push arquivo -> thiscall, ret 4), a virtual +0x24 (push arquivo -> thiscall, ret
#   4), Cup_Stage::save_to_file 0x523eb0 e League_Stage::save_to_file 0x68ca40 (push arquivo -> thiscall, ret 4), fclose
#   0x945a09 (cdecl); as caixas nas linhas 0x13e7 (nao abriu) e 0x141e (sem competicao). Os clubes da tabela, dos
#   confrontos diretos (14 bytes) e do livro de inscritos (6 bytes) viram ids e voltam. Prologo sub esp, 0x254 (6 bytes).
# Defeitos do original: (1) o nome do arquivo e montado com o nome da competicao (+4 do registro) antes de conferir se ha
#   registro: a caixa da linha 0x141e (sem competicao) nunca e alcancada, cai antes; (2) os confrontos diretos de todas as
#   linhas sao decididos pela linha 0 (+5 da primeira linha): uma tabela com confrontos so a partir da segunda linha
#   nao os grava; (3) com inscritos e sem livro (+0x14 nulo) a passagem dos ids le pelo nulo (o mesmo do Cup, cx01_7);
#   (4) a contagem do historico (+0xa7) e acertada durante a gravacao quando vale -1 (efeito colateral do save).
# Iguais no cm0102-1080.exe (inteira).
# Depende do cx01_7 (temp_write, cup_temp_file_name, cup_save_stages, club_entries_to_ids, club_entries_from_ids,
# cup_temp_file_bytes, FN_COMP_STATS_SAVE; que depende do sx16_7): integrar depois dele.
# Comparador: o arquivo gravado pela original, relido, e gravado de novo pela copia: os dois conteudos e o nome iguais.
SAVE = True
ROTINAS = [('league_save_to_temp_file', 0x6891c0, [0x81, 0xec, 0x54, 0x02, 0x00, 0x00])]
DOCS = ('o arquivo temporário de uma liga',
        'a league\'s temporary file',
        [('league_save_to_temp_file', 'League::save_to_temp_file', '0x6891c0', 'league.cpp', 'thiscall, ret 4',
          'grava o estado de uma liga no seu arquivo temporário (CM3_TEMP, pelo id da competição ou, sem ele, pelo ano, e só com a liga ativa), na ordem em que a carga o lê: o nome, as estatísticas, os dados da classe, os campos do cabeçalho, as datas das rodadas, a tabela de médias, a tabela com os confrontos diretos e o histórico de posições, as fases e o livro de inscritos (os clubes como ids)',
          "writes a league's state to its temporary file (CM3_TEMP, by the competition id or, without it, by the year, and only for an active league), in the order the load reads it: the name, the statistics, the class data, the header fields, the round dates, the points-average table, the table with its head-to-head records and position history, the stages and the entrant book (clubs as ids)")])
CAMPOS = []
INTERESSA = []
