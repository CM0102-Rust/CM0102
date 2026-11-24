# Os destrutores das eliminatorias da Eurocopa e da Copa das Confederacoes e o texto do progresso nas eliminatorias
# (euro_champ_qual.cpp, fifa_confed.cpp; leitor cx06, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do
# cx03_3/cx04_1: os destrutores rodam quando o jogo e desmontado (carregar outro save com um jogo aberto, ou sair da
# carreira); o texto do progresso e o da tela de historico de uma selecao. Caminho no jogo: com uma carreira aberta,
# abrir o historico de uma selecao europeia (Nacao > Selecao > Historico, depois de set/2002 aparece a fase das
# eliminatorias) e depois carregar outro save, jogando comparando.
# - 0x57d0a0 EuroChampQual::~EuroChampQual thiscall() ret 0: a vtable 0x96a0d4, League::free_table 0x687970 (push 0),
#   free(+0xba) e zera, as fases ate a atual (slot 0 com 1), free(+0xc) e zera (delete_stages_and_vector do cx02_3), a
#   lista dos resultados de grupo (+0x10: LLIST::~LLIST 0x68d040 e operator delete 0x944c94), as estatisticas
#   (delete_comp_stats do cx01_14), free(+0xee) sem zerar, League::~League 0x682300.
# - 0x58b4a0 FifaConfed::~FifaConfed thiscall() ret 0: o corpo das selecoes (drop_stats_league do cx04_1) com a vtable
#   0x96a270.
# - 0x57d080 e 0x58b480: os destrutores escalares (push esi; mov esi, ecx; call ~X; operator delete com o bit 0): o
#   prologo tem 3 bytes antes de um call rel32 e entra pelas RELATIVAS (0x57d080, 3) e (0x58b480, 3) do
#   rust/src/detours.rs (ver INTEGRAR_ANTES.txt); o corpo pelo delete_through do cx03_3.
# - 0x57e9d0 EuroChampQual::write_progress_text thiscall(buf, tamanho, rodada, sub, situacao, competicao) ret 0x18
#   (virtual 0x38; o ecx nao e lido): nas rodadas 0x3e8..0x3f5 (grupos), situacao 7 "Qualified For Finals", senao
#   "Group Stage"; fora delas situacao 1 "Qualified For Finals", senao "Playoff"; buffer menor que 30: a caixa (linha
#   0x31d). Prologo mov eax, [esp+8]; sub esp, 0x200 (10 bytes).
# Convencoes conferidas: free 0x9452ca e operator delete 0x944c94 (cdecl), langlib_sprintf 0x66f4e0 (cdecl), os
# destrutores thiscall, ret 0 (mov ecx -> call pelo destrutor escalar). O quadro SEH so cobre o desenrolar de excecao.
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: os destrutores so a original (liberam memoria); o texto lado a lado (o original no buffer de quem
# chama, a copia num local do mesmo tamanho).
# Depende do cx01_14 (delete_comp_stats), do cx02_3 (delete_stages_and_vector), do cx03_3 (delete_through) e do cx04_1
# (drop_stats_league): integrar depois deles.
SAVE = False
TELAS = True
ROTINAS = [('drop_euro_champ_qual', 0x57d0a0, [0x6a, 0xff, 0x68, 0x88, 0x88, 0x95, 0x00]),
           ('delete_euro_champ_qual', 0x57d080, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_fifa_confed', 0x58b4a0, [0x6a, 0xff, 0x68, 0x88, 0x8d, 0x95, 0x00]),
           ('delete_fifa_confed', 0x58b480, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('euro_champ_qual_write_progress_text', 0x57e9d0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os destrutores das eliminatórias da Eurocopa e da Copa das Confederações e o texto do progresso nas eliminatórias',
        'the destructors of the European Championship qualifiers and the Confederations Cup and the qualifiers\' progress text',
        [('drop_euro_champ_qual', 'EuroChampQual::~EuroChampQual', '0x57d0a0', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'desmonta as eliminatórias da Eurocopa: libera a tabela, o calendário, cada fase até a atual e o vetor das fases, a lista dos resultados de grupo guardados, as estatísticas e a lista de inscritos e roda o destrutor da liga',
          'destroys the European Championship qualifiers: frees the table, the round dates, each stage up to the current one and the stage vector, the stored group results, the statistics and the entrant list and runs the league destructor'),
         ('delete_euro_champ_qual', 'EuroChampQual::scalar_deleting_destructor', '0x57d080', 'euro_champ_qual.cpp', 'thiscall, ret 4',
          'o destrutor escalar das eliminatórias: o destrutor e, com o bit 0, a liberação do objeto',
          'the qualifiers\' scalar deleting destructor: the destructor and, with bit 0, the object freed'),
         ('drop_fifa_confed', 'FifaConfed::~FifaConfed', '0x58b4a0', 'fifa_confed.cpp', 'thiscall, ret 0',
          'desmonta a Copa das Confederações: libera a tabela, o calendário, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Confederations Cup: frees the table, the round dates, each stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('delete_fifa_confed', 'FifaConfed::scalar_deleting_destructor', '0x58b480', 'fifa_confed.cpp', 'thiscall, ret 4',
          'o destrutor escalar da Copa das Confederações: o destrutor e, com o bit 0, a liberação do objeto',
          'the Confederations Cup\'s scalar deleting destructor: the destructor and, with bit 0, the object freed'),
         ('euro_champ_qual_write_progress_text', 'EuroChampQual::write_progress_text', '0x57e9d0', 'euro_champ_qual.cpp', 'thiscall, ret 0x18',
          'escreve o progresso de uma seleção nas eliminatórias da Eurocopa: na fase de grupos (rodadas 0x3e8 a 0x3f5) "Classificada para a Fase Final" com a situação 7, senão "Fase de Grupos"; na repescagem "Classificada para a Fase Final" com a situação 1, senão "Repescagem"; buffer menor que 30 bytes: a caixa de erro',
          'writes a nation\'s progress in the European Championship qualifiers: in the group stage (rounds 0x3e8 to 0x3f5) "Qualified For Finals" with status 7, otherwise "Group Stage"; in the play-off "Qualified For Finals" with status 1, otherwise "Playoff"; a buffer under 30 bytes: the error box')])
CAMPOS = []
INTERESSA = []
