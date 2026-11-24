# Os dados proprios das copas da liga da Escocia e de Gales no jogo salvo (sco_lge_cup.cpp, wel_lge_cup.cpp,
# wel_prm_cup.cpp; leitor cx03, rodada 5, 24/11/2026). Item do SAVE (CARGA): Cup::load_from_disk 0x51fc00 e
# League::load_from_temp_file 0x687b10 chamam a virtual 0x20 (ler) com o FILE aberto em CM3_TEMP.
# - 0x7f0ae0 ScoLgeCup::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread 0x945bfe (4 pushes; add esp, 0x10 ->
#   cdecl) de 1 byte em +0xb2 (os isentos); curto: a caixa do sco_lge_cup.cpp (linha 0x2b8) e 0. Igual ao
#   EngCcCup::load_extra_data do cx02_13. Prologo mov eax, [esp+4]; sub esp, 0x200.
# - 0x927330 WelLgeCup::load_extra_data e 0x928c40 WelPrmCup::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread de 4
#   bytes em +0xf2 (a contagem; caixa 0x348/0x36e), malloc 4 por inscrito em +0xee (caixa 0x350/0x376), fread da lista
#   inteira (caixa 0x358/0x37e), e cada indice vira o ponteiro do clube ([0xae23b4] + i * 0x245; -1 vira 0). O mesmo
#   corpo nas duas (conferido instrucao a instrucao). Prologo sub esp, 0x200.
# AVISO DA PROVA: as tres so rodam carregando um save com as ligas da Escocia e de Gales; o FeriasTesteSave da prova do
# save e so da Inglaterra (o prova_save.py nao tem LIGAS): a prova pede um save com ENG SCO WAL, ou o Lucas carregando um
# assim comparando.
# Usa compare_cup_extra_data_load do cx02_13 (integrar depois dele).
# Iguais no cm0102-1080.exe.
# Comparadores: as leituras lado a lado com o arquivo de volta a posicao (ftell/fseek), os campos, o retorno e a posicao
# final; nas galesas a lista da copia comparada com a da original e liberada.
SAVE = True
CARGA = True
ROTINAS = [('sco_lge_cup_load_extra_data', 0x7f0ae0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wel_lge_cup_load_extra_data', 0x927330, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wel_prm_cup_load_extra_data', 0x928c40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os dados próprios das copas da liga da Escócia e de Gales no jogo salvo', "the Scottish and Welsh league cups' own data in the saved game",
        [('sco_lge_cup_load_extra_data', 'ScoLgeCup::load_extra_data', '0x7f0ae0', 'sco_lge_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Copa da Liga escocesa o número de isentos (os clubes nas copas europeias); leitura curta: a caixa de erro e 0',
          'reads from the Scottish League Cup temporary file the number of byes (the clubs in the European cups); a short read: the error box and 0'),
         ('wel_lge_cup_load_extra_data', 'WelLgeCup::load_extra_data', '0x927330', 'wel_lge_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Copa da Liga galesa a contagem e a lista dos inscritos e troca cada índice pelo clube (−1, vaga livre, vira nulo); leitura curta ou falta de memória: a caixa de erro e 0',
          'reads from the Welsh League Cup temporary file the entrant count and list and turns each index back into the club (−1, a free slot, becomes null); a short read or no memory: the error box and 0'),
         ('wel_prm_cup_load_extra_data', 'WelPrmCup::load_extra_data', '0x928c40', 'wel_prm_cup.cpp', 'thiscall, ret 4',
          'o mesmo para a Copa da Premier galesa',
          'the same for the Welsh Premier Cup')])
CAMPOS = []
INTERESSA = []
