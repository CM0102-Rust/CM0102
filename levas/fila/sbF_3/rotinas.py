# O construtor e o mata-mata da Copa da Grecia (gre_cup.cpp; leitor sbF, rodada 5, 24/11/2026). Competicoes da Grecia;
# ATE 2002-08-01 (o construtor roda na criacao do jogo; o mata-mata quando os grupos terminam, em setembro de 2001, pela
# GreCup::advance_stage do cx02_25).
# - 0x5e88a0 GreCup::GreCup thiscall(ano, registro) ret 8: o corpo do construct_irish_league_cup (conferido instrucao a
#   instrucao contra a NirLgeCup::NirLgeCup) com a vtable 0x96b4b4, 7 reservas (e nao 5), a disciplina GREECE (0x20),
#   11 fases, as rotinas da Grecia chamadas pelo endereco (0x5e9a80, 0x5ea060, 0x5e9350, 0x5e9260, 0x5e9650), 10
#   grupos e as caixas 0x56, 0x60, 0x67, 0x6d e 0x74 (o molde tem as linhas fixas 0x4f..0x6d, por isso a copia: ao
#   integrar, o construct_irish_league_cup pode receber as linhas e as reservas). SEH so para o new das estatisticas.
# - 0x5e9850 GreCup::create_knockout_stage thiscall() ret 0: o vetor de 22 (malloc 0x58, caixa 0x2a5), os 11 vices e os
#   11 primeiros (a tabela da copa e League::get_table_row 0x6826d0 das fases 0 a 9), os primeiros embaralhados
#   (comp_util_shuffle_pointer_array 0x4afb40, cdecl), o molde create_knockout_stage (ItaCup) com 22, a fase 10, sem
#   sementes, a caixa 0x2d3. SEH so para o new.
# Prologos: push -1; push imm32 (7 bytes). Iguais no cm0102-1080.exe.
# Comparadores: so a original, com o plano (os campos fixos do construtor; os 22 clubes da fase 10).
# Depende do sbF_2 so pelo endereco (o construtor chama GreCup::create_table e create_group_stage pelo endereco).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_gre_cup', 0x5e88a0, [0x6a, 0xff, 0x68, 0x49, 0xa9, 0x95, 0x00]),
           ('gre_cup_create_knockout_stage', 0x5e9850, [0x6a, 0xff, 0x68, 0xae, 0xa9, 0x95, 0x00])]
DOCS = ('o construtor e o mata-mata da Copa da Grécia',
        'the Greek Cup constructor and knockout stage',
        [('construct_gre_cup', 'GreCup::GreCup', '0x5e88a0', 'gre_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Grécia é uma copa jogada como liga com onze grupos de quatro (o grupo da própria copa e dez fases de grupo, mais uma vaga de fase para o mata-mata), 7 reservas no banco e 3 substituições, sob a disciplina grega; sem estado gravado, os 44 inscritos, as estatísticas, as posições previstas, as regras, o primeiro grupo, os seus jogos e as dez fases de grupo',
          'rule: the Greek Cup is a cup played as a league with eleven groups of four (the cup\'s own group and ten group stages, one stage slot more for the knockout), 7 substitutes on the bench and 3 substitutions, under the Greek discipline rules; with nothing saved, its 44 entrants, the statistics, the predicted positions, the rules, the first group, its fixtures and the ten group stages'),
         ('gre_cup_create_knockout_stage', 'GreCup::create_knockout_stage', '0x5e9850', 'gre_cup.cpp', 'thiscall, ret 0',
          'regra: o mata-mata da Copa da Grécia (fase 10) tem 22 clubes: os onze vices de grupo na ordem dos grupos e os onze primeiros embaralhados (doze clubes na 1ª rodada e mais dez na 2ª); o primeiro sorteio é feito logo',
          'rule: the Greek Cup knockout (stage 10) has 22 clubs: the eleven group runners-up in group order and the eleven group winners shuffled (twelve clubs in the 1st round and ten more in the 2nd); the first draw is made at once')])
CAMPOS = []
INTERESSA = []
