# Mutacoes (sbG_2), carreira ENG ate 2002-08-01 (o construtor, a tabela da fase -1 e as colocacoes rodam na criacao; as duas ultimas de novo na virada):
# - a primeira rodada com 48 selecoes no formato 0 (sem isentas): acusada pelo comparador da tabela de rodadas (lado a
#   lado);
# - a primeira faixa das colocacoes com 6 em vez de 5 selecoes: acusada pelo comparador da rank_teams (a copia de novo
#   sobre o que a original deixou);
# - o construtor sem as colocacoes previstas: as selecoes africanas nao ficam ocupadas com as eliminatorias (a copia
#   nao as marca) e entram nos amistosos internacionais da temporada [a medir na cadeia: se escapar, so os comparadores
#   das outras duas provam o item].
MUTACOES = [
    ('sbG_2_primeira_rodada_48',
     '0 => (0x32, 0x19), // sbG_2: 50 teams in the first round',
     '0 => (0x30, 0x18), // mutation: 48 teams',
     'sbG_2 african qualifying round table'),
    ('sbG_2_faixa_de_seis',
     '0 => FORMAT_0[band(&[5, 10, 15, 20, 25])], // sbG_2: bands of five',
     '0 => FORMAT_0[band(&[6, 10, 15, 20, 25])], // mutation: six in the first band',
     'sbG_2 african predicted positions'),
    ('sbG_2_sem_colocacoes',
     'call_section_routine(FN_WC_AFRICAN_CUP_RANK_TEAMS, this);',
     'let _ = FN_WC_AFRICAN_CUP_RANK_TEAMS; // mutation: no predicted positions',
     'sbG_2 african qualifying'),
]
