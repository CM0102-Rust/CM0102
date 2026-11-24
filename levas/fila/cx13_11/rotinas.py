# O play-off do titulo da Primeira Liga russa: a fase seguinte, a montagem e a atualizacao diaria (rus_prm.cpp; leitor
# cx13, rodada 5, 24/11/2026). Simulacao com a Russia (as ligas e a data dos itens mwd): a temporada russa acaba em
# novembro de 2001; o play-off so existe quando os dois primeiros empatam em pontos (0 chamadas da fase e da montagem
# na cobertura do mundo ate 2002-08-01: a prova delas e a leitura e as mutacoes que forcam o caminho).
# - 0x7eba00 RusPrm::advance_stage thiscall() ret 0: o corpo comum (advance_league_stage: caixa da linha 0x1ec na
#   ultima fase; ao chegar a 0, call direto 0x7eba90). Prologo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# - 0x7eba90 RusPrm::create_title_playoff_stage thiscall() ret 0: com os dois primeiros empatados em pontos (+0x19 das
#   linhas 0 e 1), o par, a linha 0 com +0x3a = 2, a 1 com +0x39 = 1, as duas com status 1; +0xee = pares (1 ou 0). Sem
#   par: fases 0, fase atual -1, +0xe6 = 0 e a virtual +0x94. Com par: calendario da fase 0 (virtual +0x3c), Cup_Stage
#   0x522e00 (15 argumentos, ret 0x3c) na vaga 0; sem fase a caixa da linha 0x22e (o calendario nao e liberado: vazamento
#   do original); senao free do calendario, Cup::set_up_round 0x51c800 com 0, fase atual 0 e +0xc5 = 6. O quadro SEH so
#   cobre o construtor. Prologo mov eax, fs:[0] (6 bytes).
# - 0x7ec160 RusPrm::update_for_date thiscall(data, flag) ret 8 -> u8: League::update_for_date 0x6847c0 primeiro; com
#   +0xdd, sem fases, flag e +0xe6: +0x4c = 0 e, empatados os dois primeiros, 1 fase (malloc 4) e a virtual +0x28;
#   senao +0xdd = +0xe6 = 0 e a virtual +0x94. Prologo mov eax, [esp+4]; push ebx (5 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: a fase lado a lado (compare_stage_advance); a montagem so a
# original contra o plano (pares e a fase); a atualizacao so a original.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('rus_prm_advance_stage', 0x7eba00, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('rus_prm_create_title_playoff_stage', 0x7eba90, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('rus_prm_update_for_date', 0x7ec160, [0x8b, 0x44, 0x24, 0x04, 0x53])]
DOCS = ('o play-off do título da Primeira Liga russa', 'the Russian Premier League\'s title play-off',
        [('rus_prm_advance_stage', 'RusPrm::advance_stage', '0x7eba00', 'rus_prm.cpp', 'thiscall, ret 0',
          'a fase sobe uma; na fase 0 monta o play-off do título',
          'the stage goes up by one; at stage 0 the title play-off is built'),
         ('rus_prm_create_title_playoff_stage', 'RusPrm::create_title_playoff_stage', '0x7eba90', 'rus_prm.cpp', 'thiscall, ret 0',
          'se os dois primeiros terminam empatados em pontos, o título se decide num jogo entre eles; senão as posições são refeitas sem play-off',
          'if the top two finish level on points, the title is decided in a match between them; otherwise the positions are worked out again with no play-off'),
         ('rus_prm_update_for_date', 'RusPrm::update_for_date', '0x7ec160', 'rus_prm.cpp', 'thiscall, ret 8',
          'a atualização diária comum; no fim da temporada, com empate deixado para desempate, o play-off do título se os dois primeiros empatam, senão o empate é desfeito',
          'the common daily update; at the end of the season, with a tie left to tie-break matches, the title play-off if the top two are level, otherwise the tie is dropped')])
CAMPOS = []
INTERESSA = []
