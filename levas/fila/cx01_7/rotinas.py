# O construtor do sorteio da CONMEBOL e o arquivo temporario de uma copa (conmebol_seeding.cpp, Cup.cpp; leitor cx01,
# rodada 5, 24/11/2026). Item do SAVE (CARGA): o construtor roda na criacao do jogo (create_game 0x828c89, monta a
# lista) e na carga (load_game_data 0x82bd0c, le o conmebol_seeding.tmp); o arquivo da copa e gravado a cada save.
# Usa game_cd_check do sx16_7 (integrar depois dele).
# - 0x4c5fe0 ConmebolSeeding::ConmebolSeeding thiscall() ret 0 -> this: a verificacao do CD (o jne do tipo de drive em
#   0x4c601a e o test do achado em 0x4c605d: o cm0102-1080 sem CD os troca, e a copia le esses bytes do exe que roda);
#   ConmebolSeeding::load_from_temp_file 0x4c6970 (mov ecx -> thiscall, ret 0; ja Rust, chamado pelo desvio); sem o
#   arquivo, as linhas das nacoes (Mexico e as da America do Sul, ate 11; 4 vagas Brasil e Argentina, 1 Venezuela e
#   Mexico, 3 as outras) e o campeao da Copa dos Campeoes -1.
# - 0x5210f0 Cup::save_cup_to_disk thiscall(por_id) ret 4 (slot 0x1c da Cup e das copas derivadas): o nome em +0x53,
#   CM3_TEMP "wb" (utils_build_file_path cdecl; fopen cdecl), fwrite cdecl na ordem que o Cup::load_from_disk (gra_28)
#   le; Comp_Stats::save 0x4ac4e0, a virtual +0x24, Cup_Stage::save_to_file 0x523eb0 e League_Stage::save_to_file
#   0x68ca40 (push arquivo; mov ecx -> thiscall, ret 4); os clubes da tabela de times e dos inscritos viram ids e voltam;
#   fclose cdecl. As caixas: linhas 0x147e (sem tabelas), 0x1488 (nao abriu), 0x14c3 (sem competicao).
# Iguais no cm0102-1080.exe, menos os dois bytes da verificacao do CD no construtor (a copia le os do exe que roda).
# Comparadores: o construtor so a original (CD e arquivo); o arquivo da copa gravado pela original, relido, e gravado de
# novo pela copia: os dois conteudos e o nome iguais.
# Defeito do original (0x5210f0): com inscritos e sem livro (+0x14 nulo) a passagem dos ids le pelo nulo.
SAVE = True
CARGA = True
ROTINAS = [('new_conmebol_seeding', 0x4c5fe0, [0x81, 0xec, 0x70, 0x04, 0x00, 0x00]),
           ('cup_save_cup_to_disk', 0x5210f0, [0x81, 0xec, 0x0c, 0x03, 0x00, 0x00])]
DOCS = ('o construtor do sorteio da CONMEBOL e o arquivo temporário de uma copa',
        "the CONMEBOL seeding constructor and a cup's temporary file",
        [('new_conmebol_seeding', 'ConmebolSeeding::ConmebolSeeding', '0x4c5fe0', 'conmebol_seeding.cpp', 'thiscall, ret 0',
          'pede o CD do jogo; se o sorteio volta do arquivo temporário, é só isso; senão monta as linhas das nações, na ordem da tabela, até 11: o México e as nações da América do Sul, com as vagas da Libertadores (4 para Brasil e Argentina, 1 para Venezuela e México, 3 para as outras); o campeão da Copa dos Campeões fica −1',
          'asks for the game CD; if the seeding comes back from the temporary file, that is all; otherwise builds the nation rows, in table order, up to 11: Mexico and the South American nations, with their Libertadores places (4 for Brazil and Argentina, 1 for Venezuela and Mexico, 3 for the others); the Copa dos Campeões winner becomes −1'),
         ('cup_save_cup_to_disk', 'Cup::save_cup_to_disk', '0x5210f0', 'Cup.cpp', 'thiscall, ret 4',
          'grava o estado da copa no arquivo temporário (nome da competição e id, ou o ano quando não é por id e a temporada está montada): o nome, as estatísticas, os campos, a competição, as rodadas, os times (como ids), os confrontos, as fases e os inscritos (como ids), na ordem em que a carga lê',
          "writes the cup's state into its temporary file (the competition's name and id, or the year when not by id and the season is set up): the name, the statistics, the fields, the competition, the rounds, the teams (as ids), the ties, the stages and the entrants (as ids), in the order the load reads them")])
CAMPOS = [('copa +0x53 nome do arquivo temporário (80 bytes)', 'cup +0x53 temporary file name (80 bytes)'),
          ('ConmebolSeeding linha de 10 bytes: +0 id da nação, +4 nação, +8 vagas', 'ConmebolSeeding 10-byte row: +0 nation id, +4 nation, +8 places')]
INTERESSA = []
