# As secoes regionais da Segunda Divisao sueca: a distribuicao dos 72 clubes e a nota de cada clube por secao
# (swe_prm.cpp; leitor sbG, rodada 5, 24/11/2026). Simulacao com a Suecia (LIGAS e ATE dos irmaos cx14_8/cx14_9 e mwe_14)
# e a base NORMAL: a distribuicao roda uma vez, na virada sueca do fim de 2001 (swe_prm_start_new_season, mwe_14, pelo
# endereco: call_on_league(0x890c70)); com a base MINIMUM, que o simular.py fixa, a Segunda Divisao nao existe e ela so
# mostra a caixa da linha 0x2ca (a nota nunca roda) [inferencia: os clubes ficam nas secoes, cx14_9]. Como os itens de Serie C2 do cx11 e o sbF_9: BASE='NORMAL', que
# nenhuma ferramenta le ainda.
# - 0x890c70 swe_prm_assign_second_division_sections thiscall() ret 0 (ecx = a Allsvenskan, so passado a nota): os
#   clubes (os primeiros [0xae235c] - 2 x [0xae2350] registros) com divisao +0x57 de id = o da Segunda Divisao (0x9cf7c8),
#   ate 72, e quantos tem estadio (+0x69) com cidade (+0x38); menos de 71: caixa 0x2ca (0xa8b578) e nada mais; com 71,
#   o primeiro da Divisao Inferior (0x9cf6ac) entra (+0x5b = +0x57, +0x57 o registro da Segunda Divisao, +0x60 o de
#   Gotaland do Sul). Passada com limiar de 100 descendo de 10 (a nota 0x891370 nas seis secoes, Norte, Gotaland do Sul,
#   Gotaland Leste, Gotaland Oeste, Svealand Leste, Svealand Oeste; 12 por secao) ate todos os com cidade ou os 72
#   ficarem; depois a propria secao (+0x60) e, na ultima passada, qualquer uma com vaga; a secao velha de cada um e
#   guardada, +0x60 = o registro da nova; as trocas em par e os ciclos de tres que devolvem clubes a secao velha. As
#   seis listas de 12 sao um vetor so na pilha (Norte, Sul, Svealand Leste, Svealand Oeste, Gotaland Leste, Gotaland
#   Oeste): com 71 clubes e sem um da Divisao Inferior uma vaga fica vazia e o original le um dword velho da pilha como
#   clube (defeito; a copia toma a vaga como vazia).
# - 0x891370 swe_prm_region_fit_score thiscall(clube, secao) ret 8 (o ecx, a Allsvenskan, nao e lido) -> 0..100: a
#   latitude e a longitude (+0x23, +0x2b, f64) da cidade do estadio; sem estadio ou cidade, 20; as faixas de cada secao
#   (comparacoes x87: o C0 sozinho e "abaixo" e o C0|C3 e "nao acima", de modo que um NaN fica abaixo de tudo e acima de
#   nada).
# Prologos: sub esp, 0x280 (6 bytes); push ebp; mov ebp, esp; and esp, 0xfffffff8 (6). Iguais no cm0102-1080.exe
# (inteiras).
# Comparadores: a distribuicao lado a lado sobre +0x57..+0x63 de cada clube (sem a Segunda Divisao, so a original: a
# caixa sairia duas vezes); a nota lado a lado no valor.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('swe_prm_assign_second_division_sections', 0x890c70, [0x81, 0xec, 0x80, 0x02, 0x00, 0x00]),
           ('swe_prm_region_fit_score', 0x891370, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('as seções regionais da Segunda Divisão sueca', 'the Swedish Second Division regional sections',
        [('swe_prm_assign_second_division_sections', 'swe_prm_assign_second_division_sections', '0x890c70', 'swe_prm.cpp', 'thiscall, ret 0',
          'regra: na virada sueca, os 72 clubes da Segunda Divisão (completados por um da Divisão Inferior quando há 71) são distribuídos nas seis seções regionais de doze pela localização da cidade, começando pelas notas 100 e baixando de 10 em 10; os que sobram vão para a própria seção e depois para qualquer uma com vaga; trocas em par e em ciclos de três devolvem à seção antiga os clubes que dá para devolver; com menos de 71 clubes, a caixa de erro',
          'rule: at the Swedish turnover, the 72 Second Division clubs (made up with one from the Lower Division when there are 71) are dealt into the six regional sections of twelve by where their city lies, starting at score 100 and going down by 10; those left go to their own section and then to any with room; swaps in pairs and in three-club cycles put back in their old section the clubs that can be; with fewer than 71 clubs, the error box'),
         ('swe_prm_region_fit_score', 'swe_prm_region_fit_score', '0x891370', 'swe_prm.cpp', 'thiscall, ret 8',
          'regra: a nota de 0 a 100 de um clube para uma seção regional sueca pela latitude e longitude da cidade do seu estádio (Götaland do Sul pela latitude baixa, Norte pela alta, as outras por faixas de latitude e longitude, 100 no miolo e 80 em volta); sem cidade, 20',
          "rule: a club's 0 to 100 score for a Swedish regional section by the latitude and longitude of its stadium's city (South Götaland by low latitude, North by high, the others by latitude and longitude bands, 100 in the core and 80 around it); no city, 20")])
CAMPOS = []
INTERESSA = []
