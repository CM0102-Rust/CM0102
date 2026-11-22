# O objeto das estatísticas das competições (Comp_Stats): construir, destruir e as posições projetadas de um clube
# (comp_screens.cpp; leitor tel41, rodada 2 das telas, 22/11/2026). Item de simulação: o construtor roda na criação
# de cada competição (início da carreira, 2001-07; operator new 0x5ce + construtor) e na carga das copas (gra_28); o
# destrutor quando uma competição troca de estatísticas (a carga da copa) ou some; get_club_projected_positions na
# confiança semanal da diretoria (0x69812a, 0x697c40) e no motivo da saída do técnico (0x69b472).
# Convenções conferidas na desmontagem: Comp_Stats::Comp_Stats thiscall(stats, comp) ret 4 (devolve this);
# ~Comp_Stats thiscall ret 0; get_club_projected_positions thiscall(stats, club, &best, &current, &worst) ret 0x10
# (+0x74 -> best, +0x73 -> current, +0x75 -> worst). Chamadas: os construtores dos sub-registros (0x645800, 0x448670,
# 0x49f2a0, 0x49f2f0, 0x450c60, 0x49f340, 0x453c50, 0x450d30, 0x49f390, 0x49f3f0 e 0x49f160, thiscall ret 0, só
# escrevem dentro do registro); operator new 0x944cf1 / delete 0x944c94; utils_build_file_path 0x90bde0 cdecl(dir,
# nome); FILE_LLIST::FILE_LLIST 0x58e280 thiscall(list, path, 0x26b) ret 8 e ~FILE_LLIST 0x58e330 ret 0; Date::Date
# 0x54a110 thiscall ret 0; as virtuais da competição +0x80 (tem o clube [inferência]) e +0x84
# (League::update_projected_positions, cpr_11) thiscall(stage, club) ret 4.
# Os destrutores dos membros no fim do ~Comp_Stats (0x5a53c0) são vazios: a cópia não os chama. O SEH dos construtores
# não é reproduzido (nada lança exceção).
# Comparadores: o construtor e o destrutor só a original, com o plano conferido depois (contam instâncias, abrem e
# fecham o arquivo compartilhado); a projeção lado a lado nos três bytes de saída, no +0x566 e na devolução (cada lado
# aloca e libera a sua entrada provisória; a virtual +0x84 só escreve nela, cpr_11).
# Defeitos do original: com a alocação da entrada provisória falhando, a projeção roda com +0x566 = 0 e, se der certo,
# lê os bytes do endereço 0 (o jogo cai); o destrutor libera as sequências guardadas sem zerar capacidade e contagem.
ROTINAS = [('comp_stats_construct', 0x49ee70, [0x6a, 0xff, 0x68, 0xf3, 0x64, 0x95, 0x00]),
           ('comp_stats_destruct', 0x49f450, [0x6a, 0xff, 0x68, 0x99, 0x66, 0x95, 0x00]),
           ('comp_stats_get_club_projected_positions', 0x4ae300, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o objeto das estatísticas das competições: construir, destruir e as posições projetadas de um clube',
        "the competition statistics object: building, destroying and a club's projected positions",
        [('comp_stats_construct', 'Comp_Stats::Comp_Stats', '0x49ee70', 'comp_screens.cpp', 'thiscall',
          'um objeto novo: sem tabelas, os dois conjuntos de recordes vazios, as chaves do arquivo -1, a competição em +0x502, bandeiras e contadores zerados; o primeiro objeto vivo abre o arquivo compartilhado das temporadas (comp records history.tmp, linhas de 0x26b); no fim os títulos já ganhos são contados',
          'a new object: no tables, both record sets empty, the archive keys -1, the competition at +0x502, flags and counters zeroed; the first object alive opens the shared file of the seasons (comp records history.tmp, 0x26b-byte rows); at the end the titles already won are counted'),
         ('comp_stats_destruct', 'Comp_Stats::~Comp_Stats', '0x49f450', 'comp_screens.cpp', 'thiscall',
          'libera as tabelas e as sequências guardadas; o último objeto vivo fecha o arquivo compartilhado',
          'frees the tables and the saved runs; the last object alive closes the shared file'),
         ('comp_stats_get_club_projected_positions', 'Comp_Stats::get_club_projected_positions', '0x4ae300', 'comp_screens.cpp', 'thiscall',
          'as posições projetadas (melhor, atual, pior) de um clube: a última fase da competição que tem o clube projeta numa entrada provisória (1); sem fase ou sem projeção, as da entrada do clube, quando há (0)',
          "a club's projected positions (best, current, worst): the competition's last stage that holds the club projects them into a temporary entry (1); with no stage or no projection, those of the club's entry, when there is one (0)")])
