# Os premios internacionais antigos: o objeto, os cinco premios e os dias (award/old_international_awards.cpp; leitor
# sx04c, rodada 5, 23/11/2026). Usa os auxiliares do sx05b_5, do sx03_2 e do sx04c_1
# (award_player_of_year_from_world_ratings, AWARD_MANAGER_WORLD_RATINGS): integrar depois deles.
# NAO RODAM com a base do jogo: o AWARD_MANAGER::create_nation_awards (0x41360b) so monta esta classe quando
# [0x9cff70] = -1 (base antiga); 0 chamadas na cobertura do mundo. Simulacao, sem LIGAS e sem mutacoes.
# Convencoes conferidas na desmontagem:
# - 0x79e580 thiscall(arquivo) ret 4: 0x768290; vtable 0x96e1a8; +0xe = 0 (sem nacao); +0x13 = 1; +0x12 = 4; com
#   arquivo 0x76a280, senao 0x79e5f0 (sem notas do mes);
# - 0x79e5f0 thiscall(arquivo) ret 4: +8 = 5 (word de bx), malloc(0x14) (nulo: linha 0x62); um registro de escopo de
#   14 bytes na pilha (byte 5, tres dwords -1, byte 0xa) passado ao 1o YEAR_AWARD (escopo); cinco YEAR_AWARD tipo 1,
#   competicao -1 (ebp);
# - 0x79e890 thiscall(hoje) ret 4: sai com o ano 0x7d1; Date::Date (6, 4): 0x7683c0 com as notas +0x10 (filtro 2);
#   (6, 5): 0x768b20 (hoje, premio 0) ret 8 e 0x7683c0 (premio 1, filtro 3; as notas lidas antes da 0x768b20);
#   (0x10, 5): 0x768720 (hoje, premio, notas, [continente], 0, 1, 0) ret 0x1c para Africa, Europa e America do Sul;
#   sem 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
ROTINAS = [('old_international_awards_construct', 0x79e580, [0x6a, 0xff, 0x68, 0x08, 0xee, 0x95, 0x00]),
           ('old_international_awards_init', 0x79e5f0, [0x6a, 0xff, 0x68, 0x66, 0xee, 0x95, 0x00]),
           ('old_international_awards_daily_update', 0x79e890, [0x83, 0xec, 0x10, 0x53, 0x56])]
DOCS = ('os prêmios internacionais antigos',
        'the old international awards',
        [('old_international_awards_construct', 'OldInternationalAwards::OldInternationalAwards', '0x79e580', 'award/old_international_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios internacionais antigos (só com uma base de dados sem as chaves novas): a base com a tabela virtual da classe, sem nação, o id 1 e o tipo 4; com arquivo, os salvos; senão os cinco prêmios',
          'builds the old international awards (only with a database lacking the new keys): the base with the class vtable, no nation, id 1 and type 4; with a file, the saved ones; otherwise the five awards'),
         ('old_international_awards_init', 'OldInternationalAwards::init_awards', '0x79e5f0', 'award/old_international_awards.cpp', 'thiscall, ret 4',
          'cria os cinco prêmios do ano: o jogador mundial da FIFA (com lista curta) e os melhores do mundo, da África, da Europa e da América do Sul',
          'creates the five year awards: the FIFA world player (with a shortlist) and the world, African, European and South American footballers of the year'),
         ('old_international_awards_daily_update', 'OldInternationalAwards::daily_update', '0x79e890', 'award/old_international_awards.cpp', 'thiscall, ret 4',
          'fora de 2001: em 6 de maio, o jogador mundial pelas notas mundiais; em 6 de junho, o vencedor da lista curta e o melhor do mundo; em 16 de junho, os três melhores da África, da Europa e da América do Sul; sem a atualização da base',
          'outside 2001: on 6 May, the world player from the world ratings; on 6 June, the shortlist winner and the world footballer; on 16 June, the best three of Africa, Europe and South America; no base update')])
CAMPOS = []
INTERESSA = []
