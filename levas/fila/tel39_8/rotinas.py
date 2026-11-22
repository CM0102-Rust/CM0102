# A lista de comentaristas (construtor e destruidor) e a punicao por gesto obsceno (printouts.cpp; leitor tel39,
# rodada 2 das telas, 22/11/2026).
# Convencoes conferidas na desmontagem e nos pontos de chamada:
# - 0x7da190 PunditList::PunditList: thiscall(this, lingua (byte)) ret 4 -> this; quadro SEH so pelos construtores
#   das duas listas; tabela de salto em 0x7da650 (13 linguas). Chamada pela criacao dos objetos do jogo
#   (CreateStep 0x2c, G_PUNDIT_LIST, motor.rs) e por 0x535848 (Database), 0x829f95 e 0x82cceb: roda no inicio da
#   carreira (novo jogo) e ao carregar. Chamadas: utils_build_file_path 0x90bde0, fopen 0x945b00, fclose 0x945a09,
#   strstr 0x945a60, a 0x9464f0 (fprintf da CRT [inferencia]: cdecl(arquivo, texto)), sprintf 0x944c9f,
#   PunditList::read_pundit_line 0x7da6e0 (Rust, tel36_13).
# - 0x7da690 PunditList::~PunditList: thiscall(this) ret; chamada por 0x53581a (Database, antes de refazer a lista)
#   e 0x8304b0 (Setup): so quando ja havia uma lista (carregar ou criar um segundo jogo na mesma sessao); pode nao
#   rodar na carreira de teste.
# - 0x7daf50 apply_obscene_gesture_ban: cdecl(pessoa, clube, partida) ret; chamada pelo MATCH_DAY (0x6b7712) no
#   incidente 0xb; Date::add_days 0x54c5b0, Date::operator= 0x417c10, Discipline::is_player_banned 0x55d270,
#   Discipline::add_ban 0x556940 (Rust), Friendly::is_friendly_competition 0x5c6710, 0x7570b0 (a materia da midia),
#   club_add_disliked_staff 0x547940 (Rust, cdecl). Raro: so com o bit 0x40 do +0xb do registro de estado de 79 bytes
#   da pessoa (o que docs/MARCA_DO_REGEN.md le como a marca do regen): pode nao rodar ate 2002-06-01.
# Comparadores: o construtor lado a lado (a copia monta uma lista local lendo o mesmo arquivo; listas e contador
# iguais, os bytes 0x66..0xc9 dos registros fora, lixo na original); o destruidor so a original, conferindo os
# ponteiros nulos; a punicao so a original (ban, materia e antipatia nao rodam duas vezes; add_ban e
# club_add_disliked_staff se comparam sozinhos).
# Defeitos do original: na punicao, os bits 2..7 das opcoes do registro de suspensao ficam com lixo da pilha (o
# add_ban copia os bits 0..2: o bit 2 entra no jogo); no media.out (so com -media), a linha local sem nacao poe o
# "\n" antes de ", Nation: None", e os nomes vao como formato do fprintf (um "%" no nome leria a pilha).
# Depende de: tel36_13 (pundit_list_read_pundit_line, PUNDIT_*), copy_record_date (club_records.rs), list_rewind.
ROTINAS = [('pundit_list_construct', 0x7da190, [0x6a, 0xff, 0x68, 0x4c, 0xfd, 0x95, 0x00]),
           ('pundit_list_destruct', 0x7da690, [0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x37]),
           ('apply_obscene_gesture_ban', 0x7daf50, [0x6a, 0xff, 0x68, 0x70, 0xfd, 0x95, 0x00])]
DOCS = ('a lista de comentaristas (construir e destruir) e a punição por gesto obsceno',
        'the pundit list (building and destroying it) and the obscene gesture ban',
        [('pundit_list_construct', 'PunditList::PunditList', '0x7da190', 'printouts.cpp', 'thiscall',
          'monta as listas de comentaristas nacionais e locais lendo o pundit.<língua> da pasta de dados (inglês, e as línguas 0 e 1 caem no pundit.cfg quando ele falta); com -media na linha de comando, grava o media.out com uma linha por comentarista',
          'builds the national and local pundit lists from the pundit.<language> file of the data folder (English, and languages 0 and 1 fall back to pundit.cfg when it is missing); with -media on the command line, writes media.out with one line per pundit'),
         ('pundit_list_destruct', 'PunditList::~PunditList', '0x7da690', 'printouts.cpp', 'thiscall',
          'destrói e libera as duas listas e zera os ponteiros',
          'destroys and frees both lists and clears the pointers'),
         ('apply_obscene_gesture_ban', 'apply_obscene_gesture_ban', '0x7daf50', 'printouts.cpp', 'cdecl',
          'gesto obsceno numa partida oficial: suspensão de 1 jogo a partir de daqui a 14 dias pelo órgão da competição (motivo 5), a matéria da imprensa e a antipatia do clube adversário pelo jogador; só para quem tem o bit 0x40 do +0xb do registro de estado',
          "an obscene gesture in an official match: a one-match ban starting 14 days on under the competition's ruling body (reason 5), the press story and the rival club's dislike of the player; only for one with bit 0x40 of +0xb of his state record")])
