# Os premios da Coreia do Sul: o objeto, os sete premios e o dia dos premios (award/korea_awards.cpp; leitor sx04c,
# rodada 5, 23/11/2026). Usa os auxiliares do sx05b_5, do sx03_2 e do sx04c_1 (awards_file_from_version): integrar
# depois deles.
# Rodam com a Coreia carregada: construtor e init na criacao do jogo, daily_update todo dia; os premios em 26 de
# novembro (2001-11-26); o premio do mes vira no dia do fim da temporada da nacao.
# Convencoes conferidas na desmontagem:
# - 0x66e030 thiscall(arquivo) ret 4: vtable 0x96cfd8, [KEY_NATION_SOUTH_KOREA], +0x13 = 0x1d, +0x12 = 1; com
#   arquivo 0x76a280; senao MONTH_RATINGS 0x767af0(0) em 0xf bytes (nulo: linha 0x4a e nenhum premio) e 0x66e170;
# - 0x66e170 thiscall(arquivo) ret 4: o ano de [0xae2c92] (word, empilhado como dword com o alto do quadro); +8 = 7,
#   malloc(0x1c) (nulo: linha 0x74); a equipe (sem competicao), o premio do mes (marco (2) a novembro (0xa) do mesmo
#   ano), cinco YEAR_AWARD (tipo 0, sem competicao); o artilheiro (opcao 1) e o de mais assistencias recebem o arquivo
#   so se |[0xdd7110]| >= 0x3e047; as chaves 0x9d0084..0x9d00a0 (sem nome no keys.rs: ficam cruas com comentario);
# - 0x66e4d0 thiscall(hoje) ret 4: Date::Date (0x1a, 0xa, ano, -1); 0x768940 (tecnico da K-League, premio 2); quatro
#   0x769380 (hoje, premio, 7 filtros) ret 0x24 com a Coreia no 1o filtro; 0x7698f0 (K-League, premio 0); o dia do
#   fim da temporada (0x669340, dia com sinal contra a palavra sem sinal) passa o premio 1; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ROTINAS = [('korea_awards_construct', 0x66e030, [0x6a, 0xff, 0x68, 0x09, 0xc8, 0x95, 0x00]),
           ('korea_awards_init', 0x66e170, [0x6a, 0xff, 0x68, 0x82, 0xc8, 0x95, 0x00]),
           ('korea_awards_daily_update', 0x66e4d0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Coreia do Sul',
        'the South Korean awards',
        [('korea_awards_construct', 'KoreaAwards::KoreaAwards', '0x66e030', 'award/korea_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Coreia do Sul: a base com a tabela virtual da classe, a Coreia do Sul, o id 0x1d e o tipo 1; com arquivo, os salvos; senão as notas do mês (sem memória, a caixa de erro e nenhum prêmio) e os sete prêmios',
          'builds the South Korean awards: the base with the class vtable, South Korea, id 0x1d and type 1; with a file, the saved ones; otherwise the month ratings (out of memory, the error box and no awards) and the seven awards'),
         ('korea_awards_init', 'KoreaAwards::init_awards', '0x66e170', 'award/korea_awards.cpp', 'thiscall, ret 4',
          'cria os sete prêmios coreanos, sem competição: a seleção do ano, o jogador do mês (de março a novembro), técnico, jogador, artilheiro, jovem e o de mais assistências do ano; artilheiro e assistências só vêm do arquivo se o save for da versão 0x3e047 em diante',
          'creates the seven Korean awards, with no competition: the best 11 of the year, the player of the month (March to November), manager, player, top goalscorer, young player and most assisted player of the year; top goalscorer and assists come from the file only when the save is version 0x3e047 or later'),
         ('korea_awards_daily_update', 'KoreaAwards::daily_update', '0x66e4d0', 'award/korea_awards.cpp', 'thiscall, ret 4',
          'em 26 de novembro, o técnico do ano da K-League, jogador, jovem, artilheiro e o de mais assistências do ano pelos três melhores das notas do ano, e a seleção do ano pela equipe da temporada; no dia do fim da temporada, o prêmio do mês passa à temporada seguinte; depois, todo dia, a atualização da base',
          'on 26 November, the K-League manager of the year, player, young player, top goalscorer and most assisted player of the year from the year ratings top three, and the best 11 from the team of the season; on the season end day, the month award moves on; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
