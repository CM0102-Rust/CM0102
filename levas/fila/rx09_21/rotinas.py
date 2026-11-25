# A fase de liga feita e lida do jogo salvo (league_stage.cpp; leitor rx09, rodada 6, 25/11/2026). Item do SAVE (CARGA),
# no molde do cx07_10: a carga do arquivo temporario de uma competicao (League::load_from_temp_file, ja em Rust,
# league_load_stages) faz cada fase de liga pelo construtor 0x68b4f0 (new 0xee) e a le pela 0x68b720. As fases de grupo
# das copas europeias da carreira ENG estao no save de ferias.
# - 0x68b4f0 League_Stage::League_Stage thiscall(clubes, rodadas) ret 8 (o quadro SEH so do desenrolar): League::League,
#   vtable 0x96d0f0, malloc 59 x clubes (caixa 0xa6); rodadas > 0 (palavra com sinal): malloc 0x41 x rodadas (caixa
#   0xaf); senao malloc 0x41 (caixa 0xba) com a data de hoje ([0xae2c90], [0xae2c92] - ano, 0). Prologo push -1; push
#   imm32 (7 bytes).
# - 0x68b720 League_Stage::load_from_file thiscall(arquivo) ret 4 -> 0/1 (4,9 KB, sem tabela de saltos): os campos lidos
#   um a um na ordem conferida (as caixas fatais 0xff..0x1ad: caixa, game_fatal_cleanup 0x5cc7d0, 0x5a8f60 e exit(-1)
#   0x945934; o +0x3a sem conferir); o registro da competicao pelo id ([0xae23d0] + 0x6b x id; -1: nenhum); as datas das
#   rodadas no vetor do construtor (0x1b4 fatal); +0x18 e +0x34 sem conferir e o vetor de 0x13 bytes; as linhas
#   (0x1cc fatal), a marca do confronto direto (0x1d4 fatal) e os vetores de 0xe x (n - 1) (0x1e2, 0x1eb: caixa e 0); a
#   marca do historico (0x1f7: caixa e 0); +0xa7 = (n - 1) x +0x3c quando -1 (sempre, antes da marca); os historicos
#   (0x20c, 0x215); os ids das linhas e dos adversarios (pela marca da linha 0) de volta a ponteiros. Prologo sub esp,
#   0x214 (6 bytes).
# Iguais no cm0102-1080.exe. Comparadores: so a original nas duas (alocam e leem o arquivo); a fase feita (vtable,
# tabela, datas) e a lida (1 e a competicao do banco) conferidas depois; a prova e a do save (carga com e sem a leva).
SAVE = True
CARGA = True
ROTINAS = [('construct_league_stage_for_load', 0x68b4f0, [0x6a, 0xff, 0x68, 0x0b, 0xcb, 0x95, 0x00]),
           ('league_stage_load_from_file', 0x68b720, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00])]
DOCS = ('a fase de liga feita e lida do jogo salvo', 'the league stage made and read back from the saved game',
        [('construct_league_stage_for_load', 'League_Stage::League_Stage', '0x68b4f0', 'league_stage.cpp', 'thiscall, ret 8',
          'monta a fase de liga vazia que a carga do save lê: a tabela dos clubes e as datas das rodadas; sem rodadas, uma rodada com a data de hoje',
          'builds the empty league stage the save loader reads into: the club table and the round dates; without rounds, one round dated today'),
         ('league_stage_load_from_file', 'League_Stage::load_from_file', '0x68b720', 'league_stage.cpp', 'thiscall, ret 4',
          'lê uma fase de liga do save campo a campo: as regras, a competição pelo id, as datas, as linhas da tabela, os confrontos diretos e o histórico de posições; os ids dos clubes voltam a ser os clubes; uma leitura curta dos campos para o jogo',
          'reads a league stage from the save field by field: the rules, the competition by its id, the dates, the table rows, the head-to-head entries and the position history; the club ids become the clubs again; a short read of the fields stops the game')])
CAMPOS = []
INTERESSA = []
