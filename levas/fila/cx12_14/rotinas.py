# O calendario e a tabela da liga coreana (kor_league.cpp; leitor cx12, rodada 5, 24/11/2026). Simulacao com a Coreia
# (LIGAS, ATE do mwf_11): a cobertura do mundo (docs/cobertura_mundo.txt, ate 2002-08-01) da 2 chamadas a cada uma (a
# criacao do jogo, temporada de 2001, e a virada do fim de 2001, temporada de 2002 a partir de 28 de julho).
# - 0x66b130 KorLeague::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10 (virtual 0x3c): so a fase
#   -1: *e_liga = 1 (quando dado), 27 rodadas, extra 0, malloc 0x6db (sem memoria a caixa da linha 0xa1 e 0); com o ano
#   2002 (cmp ax, 0x7d2) as datas de 28/7 a 27/10 (a rodada 0 com o ano literal 2002), senao de 16/6 a 27/10;
#   league_round_set_date 0x68a160 e league_round_set_match_slot 0x68a1c0 (cdecl, cada rodada com a vaga 0 terminada):
#   fra_third_write_league_rounds. Tabelas extraidas pelo emulador de caminhos e conferidas. Prologo mov al, [esp+4];
#   sub esp, 0x200 (10 bytes).
# - 0x66bba0 KorLeague::create_table thiscall() ret 0: 10 clubes, malloc 0x24e (sem memoria a caixa da linha 0x1a1 e
#   0); cada clube cujo ponteiro de divisao (+0x57) e o registro da propria competicao (+4) entra na linha seguinte
#   (League::init_table_row 0x687430, thiscall(linha, clube, 0, 0)), parando quando enche; sem encher, 0; cheia, o fundo
#   de premios com a cota 10.000 (League::calc_prize_fund 0x682f70) e +0xe1 = 1; 1. Diferente do molde do NirPrm
#   (mwa_9): compara o ponteiro, para ao encher e nao tem a linha da caixa dele. Prologo sub esp, 0x200 (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: o calendario lado a lado (compare_c1_schedule); a tabela so a
# original contra o plano (compare_japanese_table, pela chave da liga coreana).
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('kor_league_create_schedule', 0x66b130, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('kor_league_create_table', 0x66bba0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário e a tabela da liga coreana', 'the Korean league schedule and table',
        [('kor_league_create_schedule', 'KorLeague::create_schedule', '0x66b130', 'kor_league.cpp', 'thiscall, ret 0x10',
          'o calendário da K League: 27 rodadas; em 2002 de 28 de julho a 27 de outubro, nos outros anos de 16 de junho a 27 de outubro',
          'the K League schedule: 27 rounds; in 2002 from 28 July to 27 October, in other years from 16 June to 27 October'),
         ('kor_league_create_table', 'KorLeague::create_table', '0x66bba0', 'kor_league.cpp', 'thiscall, ret 0',
          'a tabela da K League: os 10 primeiros clubes da liga na ordem do banco; cheia, o fundo de prêmios com a cota de 10.000 por lugar',
          "the K League table: the league's first 10 clubs in database order; when full, the prize fund with a 10,000 share per place")])
CAMPOS = []
INTERESSA = []
