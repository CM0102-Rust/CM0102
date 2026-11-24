# Montar e liberar os grupos das estatisticas simuladas (simulated_stats.cpp; leitor sx20, rodada 5, 24/11/2026):
# SimulatedStats::SimulatedStats (0x84b940) e SimulatedStats::~SimulatedStats (0x84be10). Rotinas inteiras
# (0x84b940..0x84be06 ret 4; 0x84be10..0x84be1e ret). Quem chama: a criacao do jogo e a passada das temporadas
# (0x5d5ae0, nma_2: delete do antigo, new(8) e o construtor com save 0), a carga do save (com o save).
# DEPENDE DO sx20_1 (integrar junto ou depois): usa simulated_group, simulated_nearby_index e SIMULATED_GROUP_*.
# Convencoes conferidas na desmontagem:
# - 0x84b940 thiscall(this, save) ret 4 -> this; 0x84be10 thiscall(this) ret (eax sem valor);
# - a busca do CD so com [0xdd7118] (G_INTERACTIVE_MODE_FLAG) = 0: GetLogicalDriveStringsA ([0x9670f8], 200, buf),
#   GetDriveTypeA ([0x967100]) = 5, sprintf(nome, "%s%c%c%c%c%c%c%c%c%c%c", raiz, 'c','m','0','1','0','2','.','e','x',
#   'e'), 0x4c0aa0 file_find_first cdecl(nome, achado, 0) -> 0 achou; senao 0x4c0c10 file_find_close cdecl(achado);
#   nenhum: 0x66f4e0 langlib_sprintf ("Insert CD", "Please insert the CM01/02 CD and click on OK") e a caixa
#   0x5e8290(titulo, texto, 1), e de novo. No cm0102-1080.exe o jne de 0x84b9a0 vira nop e o test de 0x84b9e3 vira
#   xor (sem CD): a copia le esses bytes, como a club_history_find_cd;
# - com save: 0x84c510 SimulatedStats::load thiscall(this, save) ret 4;
# - 0x944e46 malloc(clubes * 4); sem memoria: a caixa (simulated_stats.cpp 0x49), 0x5cc7d0 game_fatal_cleanup,
#   0x5a8f60 e 0x945934 exit(-1); 0x775790 NationalTeams::is_national_or_all_star_team thiscall([0xb74318], clube)
#   ret 4; 0x90cfc0 utils_random cdecl(n); 0x944e46 malloc(grupos * 0x44) sem conferir; 0x9452ca free.
# Defeitos do original: com menos de 16 x (16 - n % 16) clubes os grupos curtos pedem mais clubes do que ha (a caixa
# da linha 0x95 uma vez e ids lidos alem da lista); a tabela de grupos nao e conferida.
ROTINAS = [('simulated_stats_simulated_stats', 0x84b940, [0x81, 0xec, 0xfc, 0x04, 0x00, 0x00]),
           ('simulated_stats_destructor', 0x84be10, [0x8b, 0x41, 0x04, 0x85, 0xc0])]
DOCS = ('montar e liberar os grupos das estatisticas simuladas',
        'building and freeing the simulated statistics groups',
        [('simulated_stats_simulated_stats', 'SimulatedStats::SimulatedStats', '0x84b940', 'simulated_stats.cpp', 'thiscall, ret 4',
          'monta as estatisticas simuladas: espera o CD (sem o modo interativo); com save, carrega os grupos dele; senao junta os clubes fora das competicoes jogadas (divisao e divisao reserva sem objeto) que nao sao selecoes, o numero de grupos = teto(n / 16) em +0, ordena pela reputacao (maior primeiro), embaralha um pouco (n / 2 trocas de um clube ao acaso com um vizinho, ate n / 150 para cada lado) e distribui em ordem numa tabela nova de grupos em +4: 16 por grupo, salvo os primeiros 16 - (n % 16) grupos, com 15, quando n nao e multiplo de 16; devolve this',
          'builds the simulated statistics: waits for the CD (without the interactive mode); with a save, loads the groups from it; otherwise takes the clubs outside the played competitions (division and reserve division without an object) that are not national teams, the group count = ceil(n / 16) at +0, sorts them by reputation (highest first), shuffles them a little (n / 2 swaps of a random club with a neighbour, up to n / 150 either side) and deals them in order into a new group table at +4: 16 per group, except the first 16 - (n % 16) groups, with 15, when n is not a multiple of 16; returns this'),
         ('simulated_stats_destructor', 'SimulatedStats::~SimulatedStats', '0x84be10', 'simulated_stats.cpp', 'thiscall',
          'libera a tabela de grupos (+4), se houver',
          'frees the group table (+4), when there is one')])
