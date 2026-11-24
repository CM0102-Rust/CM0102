# O grupo B e o mata-mata da Copa da Liga coreana (kor_league_cup.cpp; leitor sbF, rodada 5, 24/11/2026). Competicoes
# da Coreia; ATE 2002-08-01 (o grupo B na criacao, pelo construtor KorLeagueCup do mwf_22; o mata-mata no fim dos
# grupos, em abril de 2002, pela KorLeagueCup::advance_stage).
# - 0x66cb60 KorLeagueCup::create_group_b_stage thiscall(g) ret 4 -> 1/0: o corpo da WelPrmCup::create_group_stage
#   (conferido instrucao a instrucao) com a fase sempre 0 (o argumento nao e lido), 5 clubes das vagas impares da lista
#   (zeradas), o 18o argumento do League_Stage 0x82 e as caixas 0x245 e 0x25b. Usa o build_cup_group_stage do sbF_2.
# - 0x66cd50 KorLeagueCup::create_knockout_stage thiscall() ret 0: o vetor de 4 (malloc 0x10, caixa 0x279): o 1o do
#   grupo A, o 2o do grupo B (a tabela da fase 0 lida direto), o 2o do A, o 1o do B; o molde create_knockout_stage
#   (ItaCup) com 4, a fase 1, sem sementes, a caixa 0x290 (conferido contra a NirLgeCup::create_knockout_stage).
# Prologos: push -1; push 0x95c76e (7 bytes); mov eax, fs:[0] (6 bytes). Iguais no cm0102-1080.exe.
# Comparadores: so a original, com o plano (os clubes do grupo B e as vagas zeradas; os 4 clubes da fase 1).
# Depende do sbF_2 (build_cup_group_stage, CupGroupStage): integrar depois dele.
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('kor_league_cup_create_group_b_stage', 0x66cb60, [0x6a, 0xff, 0x68, 0x6e, 0xc7, 0x95, 0x00]),
           ('kor_league_cup_create_knockout_stage', 0x66cd50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o grupo B e o mata-mata da Copa da Liga coreana',
        'the Korean League Cup group B and knockout stages',
        [('kor_league_cup_create_group_b_stage', 'KorLeagueCup::create_group_b_stage', '0x66cb60', 'kor_league_cup.cpp', 'thiscall, ret 4',
          'regra: o grupo B da Copa da Liga coreana é uma fase de liga com os cinco cabeças ímpares (as vagas 1, 3, 5, 7 e 9 da lista), que saem da lista, no calendário de grupos da copa; depois a notícia do sorteio do grupo',
          'rule: the Korean League Cup group B is a league stage of the five odd seeds (list places 1, 3, 5, 7 and 9), which leave the list, on the cup\'s group schedule; then the group draw news'),
         ('kor_league_cup_create_knockout_stage', 'KorLeagueCup::create_knockout_stage', '0x66cd50', 'kor_league_cup.cpp', 'thiscall, ret 0',
          'regra: as semifinais da Copa da Liga coreana cruzam os grupos: o 1º do grupo A contra o 2º do B e o 2º do A contra o 1º do B; o primeiro sorteio é feito logo',
          'rule: the Korean League Cup semi-finals cross the groups: group A\'s winner against group B\'s runner-up and group A\'s runner-up against group B\'s winner; the first draw is made at once')])
CAMPOS = []
INTERESSA = []
