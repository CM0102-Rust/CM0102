# As fases de grupo e o mata-mata da Copa da Holanda (hol_cup.cpp; leitor sbF, rodada 5, 24/11/2026). Competicoes da
# Holanda; ATE 2002-08-01 (os grupos no primeiro dia completo da temporada, HolCup::update_for_date do sbF_7; o
# mata-mata quando os grupos de agosto terminam, pela HolCup::advance_stage).
# - 0x5f4220 HolCup::create_group_stage thiscall(g) ret 4 -> 1/0: o corpo da WelPrmCup::create_group_stage com os
#   clubes por varredura (as proximas entradas nao nulas do pool, zeradas; sem limite), 4 clubes (3 e o desempate 1 na
#   fase 18) e o 4o argumento do League_Stage 1; caixas 0x280 e 0x299. Usa o build_cup_group_stage do sbF_2.
# - 0x5f4480 HolCup::create_knockout_stage thiscall() ret 0: dois vetores de 46 (malloc 0xb8; caixas 0x2b9 e 0x2c1;
#   sem liberar o primeiro quando o segundo falha); os 20 primeiros e os 19 vices (so das fases de 4 clubes); um
#   primeiro sorteado (utils_random 0x90cfc0, cdecl) passa aos vices e o seu lugar recebe o inscrito f2-7; 20 pares
#   sorteados (primeiro x vice, o mando por utils_random(2)); os 6 ultimos inscritos; o Cup_Stage da fase 19 (46
#   clubes; o molde create_knockout_stage da ItaCup escrito inteiro, porque o original libera tres blocos, nesta
#   ordem: o vetor, o sorteio e a tabela); caixa 0x30e; o primeiro sorteio (Cup::draw_round(0)).
# Prologos: push -1; push 0x95acde (7 bytes); mov eax, fs:[0] (6 bytes). Iguais no cm0102-1080.exe.
# Comparadores: so a original, com o plano (os clubes da fase e as entradas zeradas; os 46 do mata-mata como conjunto).
# Depende do sbF_2 (build_cup_group_stage, CupGroupStage, cup_pool_entry): integrar depois dele.
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('hol_cup_create_group_stage', 0x5f4220, [0x6a, 0xff, 0x68, 0xde, 0xac, 0x95, 0x00]),
           ('hol_cup_create_knockout_stage', 0x5f4480, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as fases de grupo e o mata-mata da Copa da Holanda',
        'the Dutch Cup group stages and knockout stage',
        [('hol_cup_create_group_stage', 'HolCup::create_group_stage', '0x5f4220', 'hol_cup.cpp', 'thiscall, ret 4',
          'regra: a fase de grupo g (0 a 18) da Copa da Holanda é uma fase de liga com os quatro próximos inscritos que ainda estão no pool (três no último grupo, a fase 18), que saem do pool; depois a notícia do sorteio do grupo e a fase atual passa a g',
          'rule: Dutch Cup group stage g (0 to 18) is a league stage of the next four entrants still in the pool (three in the last group, stage 18), which leave the pool; then the group draw news and the current stage becomes g'),
         ('hol_cup_create_knockout_stage', 'HolCup::create_knockout_stage', '0x5f4480', 'hol_cup.cpp', 'thiscall, ret 0',
          'regra: o mata-mata da Copa da Holanda (fase 19) tem 46 clubes: 20 confrontos sorteados de um primeiro de grupo contra um segundo (um primeiro sorteado passa para o lado dos segundos e o seu lugar fica com o sétimo inscrito antes do fim; o mando sorteado) e os seis últimos inscritos (as copas europeias), que entram na 3ª rodada; o primeiro sorteio é feito logo',
          'rule: the Dutch Cup knockout (stage 19) has 46 clubs: 20 drawn ties of a group winner against a runner-up (one drawn winner crosses to the runners-up side and its place goes to the seventh entrant from the end; the home side drawn) and the last six entrants (the European cups), who enter in the 3rd round; the first draw is made at once')])
CAMPOS = []
INTERESSA = []
