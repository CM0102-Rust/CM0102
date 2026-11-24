# Os filtros da busca de jogadores: padroes, recomecos, os sete valores simples e os minimos de atributo
# (search_filters.cpp; leitor sx20, rodada 5, 24/11/2026): 0x806450 a 0x808280, fora set_option_flag (0x806630,
# 2,5 KB) e player_matches_filter (0x807450, 3,4 KB), que ficam originais (chamados pelo endereco). Rotinas inteiras
# (0x806450..0x8064c9 ret; 0x8064d0..0x8065c5 ret; 0x8065d0..0x806626 ret; cada setter de 0x806ff0 a 0x8073b0 ate o
# seu ret 4, 0xa0 bytes; 0x808190..0x808272 ret 8; 0x808280..0x8084b1 ret 0xc).
# TELAS: a prova e o Lucas jogando comparando: a tela de busca de jogadores (Buscar), mudando os filtros: nacionalidade
# e onde joga (nacao e regiao), idioma, faixa de valor, cargo (busca de comissao), os minimos de atributo de jogador e
# de nao jogador (subir de 1 e voltar a 1), e os botoes de recomecar os filtros gerais e os de atributos. Na carreira
# automatica os filtros nao rodam (0 chamadas na cobertura do mundo).
# Convencoes conferidas na desmontagem (thiscall sobre o filtro de 0x80 bytes):
# - 0x806450 / 0x8064d0 / 0x8065d0(filtro) ret; 0x806630 set_option_flag thiscall(filtro, baixo, alto) ret 8 (os dois
#   dwords da mascara de 64 bits; os quinze padroes do 0x8064d0 na ordem da original);
# - os setters (filtro, valor) ret 4 -> 1/0: nacoes dword de -1 a [0xae2350] - 1, os outros byte com sinal; fora da
#   faixa a caixa (search_filters.cpp 0x14a, 0x161, 0x178, 0x18f, 0x1a6, 0x1bd, 0x1d4) e 0;
# - 0x808190(filtro, atributo, nao_jogador) ret 8 -> al (fora da faixa a caixa 0x3c6 / 0x3d2 e 1);
# - 0x808280(filtro, atributo, valor, nao_jogador) ret 0xc -> 1/0 (fora da faixa a caixa 0x3ec / 0x420 e 0).
# Comparadores: lado a lado sobre o filtro (0x80 bytes) e a resposta; so a original quando o valor abre a caixa.
TELAS = True
ROTINAS = [('search_filters_set_defaults', 0x806450, [0x33, 0xc0, 0xc7, 0x41, 0x08, 0x10, 0x00, 0x00, 0x00]),
           ('search_filters_reset_general_filters', 0x8064d0, [0x53, 0x33, 0xdb, 0x56, 0x53]),
           ('search_filters_reset_attribute_filters', 0x8065d0, [0x53, 0x56, 0x68, 0x00, 0x00, 0x02, 0x00]),
           ('search_filters_set_nationality_nation', 0x806ff0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('search_filters_set_nationality_region', 0x807090, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('search_filters_set_language_option', 0x807130, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('search_filters_set_based_nation', 0x8071d0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('search_filters_set_based_region', 0x807270, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('search_filters_set_value_band', 0x807310, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('search_filters_set_job_filter', 0x8073b0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('search_filters_get_attribute_minimum', 0x808190, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('search_filters_set_attribute_minimum', 0x808280, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
_F = 'search_filters.cpp'
DOCS = ('os filtros da busca de jogadores: padroes, recomecos, valores simples e minimos de atributo',
        'the player search filters: defaults, resets, single values and attribute minimums',
        [('search_filters_set_defaults', 'SearchFilters::set_defaults', '0x806450', _F, 'thiscall',
          'grava o padrao de todo filtro: as mascaras 0x14140081 / 0x44aaa884, sem nacionalidade nem onde joga (nacao e regiao), todo idioma, qualquer faixa de valor, cargo 5, os dwords de opcao +8 = 0x10 e +0x10 = 0x800000, as duas tabelas de atributo de 1 a 20 com os contadores zerados e +0x7f zerado',
          'writes every filter default: the masks 0x14140081 / 0x44aaa884, no nationality or based-in nation and region, every language, any value band, job 5, option dwords +8 = 0x10 and +0x10 = 0x800000, both attribute tables 1 to 20 with their counters cleared and +0x7f cleared'),
         ('search_filters_reset_general_filters', 'SearchFilters::reset_general_filters', '0x8064d0', _F, 'thiscall',
          'recomeca os filtros gerais: o bit padrao de cada grupo de opcao (quinze set_option_flag), os padroes gerais e +0x7f; as tabelas de atributo ficam',
          'resets the general filters: the default bit of each option group (fifteen set_option_flag), the general defaults and +0x7f; the attribute tables stay'),
         ('search_filters_reset_attribute_filters', 'SearchFilters::reset_attribute_filters', '0x8065d0', _F, 'thiscall',
          'recomeca as duas tabelas de atributo (de 1 a 20, contadores zerados), com os bits de opcao das faixas (alto 0x20000 e 0x80000)',
          'resets both attribute tables (1 to 20, counters cleared), with the range option bits (high 0x20000 and 0x80000)'),
         ('search_filters_set_nationality_nation', 'SearchFilters::set_nationality_nation', '0x806ff0', _F, 'thiscall, ret 4',
          'guarda a nacao de nacionalidade quando esta de -1 ao numero de nacoes - 1 e devolve 1; senao a caixa (0x14a) e 0',
          'stores the nationality nation when it lies from -1 to the nation count less one and returns 1; otherwise the box (0x14a) and 0'),
         ('search_filters_set_nationality_region', 'SearchFilters::set_nationality_region', '0x807090', _F, 'thiscall, ret 4',
          'guarda a regiao de nacionalidade de 1 a 14 e devolve 1; senao a caixa (0x161) e 0',
          'stores the nationality region from 1 to 14 and returns 1; otherwise the box (0x161) and 0'),
         ('search_filters_set_language_option', 'SearchFilters::set_language_option', '0x807130', _F, 'thiscall, ret 4',
          'guarda a opcao de idioma de 1 a 3 e devolve 1; senao a caixa (0x178) e 0',
          'stores the language option from 1 to 3 and returns 1; otherwise the box (0x178) and 0'),
         ('search_filters_set_based_nation', 'SearchFilters::set_based_nation', '0x8071d0', _F, 'thiscall, ret 4',
          'guarda a nacao onde joga de -1 ao numero de nacoes - 1 e devolve 1; senao a caixa (0x18f) e 0',
          'stores the based-in nation from -1 to the nation count less one and returns 1; otherwise the box (0x18f) and 0'),
         ('search_filters_set_based_region', 'SearchFilters::set_based_region', '0x807270', _F, 'thiscall, ret 4',
          'guarda a regiao onde joga de 1 a 14 e devolve 1; senao a caixa (0x1a6) e 0',
          'stores the based-in region from 1 to 14 and returns 1; otherwise the box (0x1a6) and 0'),
         ('search_filters_set_value_band', 'SearchFilters::set_value_band', '0x807310', _F, 'thiscall, ret 4',
          'guarda a faixa de valor de -1 a 7 e devolve 1; senao a caixa (0x1bd) e 0',
          'stores the value band from -1 to 7 and returns 1; otherwise the box (0x1bd) and 0'),
         ('search_filters_set_job_filter', 'SearchFilters::set_job_filter', '0x8073b0', _F, 'thiscall, ret 4',
          'guarda o cargo de 1 a 9 e devolve 1; senao a caixa (0x1d4) e 0',
          'stores the job from 1 to 9 and returns 1; otherwise the box (0x1d4) and 0'),
         ('search_filters_get_attribute_minimum', 'SearchFilters::get_attribute_minimum', '0x808190', _F, 'thiscall, ret 8',
          'devolve o minimo do atributo: da tabela de nao jogador (1 a 12) com o terceiro argumento, senao da de jogador (1 a 31); fora da faixa a caixa (0x3c6 / 0x3d2) e 1',
          'returns the attribute minimum: of the non-player table (1 to 12) with the third argument, else of the player one (1 to 31); out of range the box (0x3c6 / 0x3d2) and 1'),
         ('search_filters_set_attribute_minimum', 'SearchFilters::set_attribute_minimum', '0x808280', _F, 'thiscall, ret 0xc',
          'poe o minimo do atributo de 1 ao maximo dele (fora disso a caixa 0x3ec / 0x420 e 0; o mesmo valor, 0); num atributo de faixa inteira (maximo 20) a primeira subida a partir de 1 conta a faixa (os dois contadores) e liga a opcao de faixas da tabela (alto 0x100000 nao jogador, 0x40000 jogador); voltar a 1 descota (o contador de postas, o de mostradas enquanto acima de 1) e, sem nenhuma, zera o de mostradas e liga a opcao sem faixas (0x80000 / 0x20000); devolve 1',
          'sets the attribute minimum from 1 to its maximum (otherwise the box 0x3ec / 0x420 and 0; the same value, 0); on a full-range attribute (maximum 20) the first raise from 1 counts the range (both counters) and switches the table range option on (high 0x100000 non-player, 0x40000 player); going back to 1 counts it off (the set counter, the shown one while above 1) and, with none left, clears the shown one and switches the no-ranges option (0x80000 / 0x20000); returns 1')])
