# Os play-offs das divisões inglesas (eng_first.cpp, eng_second.cpp, eng_third.cpp; a família de league.cpp/Cup.cpp
# está quase esgotada e as ligas inglesas não têm dono na fila): EngFirst::create_playoff_stage (0x573080),
# EngSecond::create_play_off_stage (0x576a70) e EngThird::create_play_off_stage (0x5783c0), chamadas pela fase
# seguinte de cada uma (0x572ff0, 0x5769e0, 0x578330: a virtual +0x28) quando a fase passa de -1 a 0, no fim da
# temporada regular (maio de 2002). Montam a lista dos 4 do mata-mata pela tabela (First e Second: 6º, 3º, 5º e 4º;
# Third: 7º, 4º, 6º e 5º), a tabela de rodadas (virtual +0x3c), a fase (Cup_Stage::Cup_Stage 0x522e00) e a rodada
# (0x51c800, Rust). Quadro SEH só no construtor da fase (sem destrutor). Iguais no cm0102-1080.exe.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl(n).
# - virtual +0x3c (create_round_table): push 0 (cedo, 0x5730f1); push &extra; push &rodadas; push 0; ecx = this; o
#   chamado limpa 0x10 -> thiscall(this, 0, &rodadas, &extra, nulo), ret 0x10.
# - 0x944cf1 (operator new): push 0xb2; call; add esp, 4 -> cdecl(n).
# - 0x522e00 (Cup_Stage::Cup_Stage): 15 pushes; ecx = o novo; sem add esp (ret 0x3c) -> thiscall com 15 argumentos.
# - 0x9452ca (free): push p; call; ... add esp, 8 depois das duas -> cdecl(p).
# - 0x51c800 (Cup::set_up_round): push 0; ecx = a fase; sem add esp -> thiscall(fase, 0), ret 4.
# - a caixa de erro (aviso_em do motor.rs: 0x944cff cdecl 5, 0x90d130 cdecl 3, 0x944c9f cdecl, 0x5e8290 cdecl 3).
# Comparadores: as três alocam (só a original roda); o plano, feito antes, são os clubes das 4 linhas, que têm de ser
# os da lista (+0xa7) da fase nova (como conjunto), com +0x3e = a conta (+0xbf) e +4 = o da liga.
SAVE = False
ROTINAS = [('mata_mata_da_first', 0x573080, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('mata_mata_da_second', 0x576a70, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('mata_mata_da_third', 0x5783c0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os play-offs das divisões inglesas: o mata-mata da First, da Second e da Third',
        'the English divisions play-offs: the First, Second and Third play-off stages',
        [('mata_mata_da_first', 'EngFirst::create_playoff_stage', '0x573080', 'eng_first.cpp', 'thiscall, ret 0',
          'monta a fase do mata-mata com o 6º, o 3º, o 5º e o 4º da tabela (chaves 6º × 3º e 5º × 4º), a tabela de rodadas da virtual +0x3c e +0x3a = 0xa0; põe a fase na primeira vaga (+0xc) e monta a rodada (0x51c800); sem memória, a caixa (linhas 0x2f0 e 0x306)',
          'builds the play-off stage with the 6th, 3rd, 5th and 4th of the table (ties 6th v 3rd and 5th v 4th), the round table from virtual +0x3c and +0x3a = 0xa0; puts the stage in the first slot (+0xc) and sets up the round (0x51c800); out of memory, the error box (lines 0x2f0 and 0x306)'),
         ('mata_mata_da_second', 'EngSecond::create_play_off_stage', '0x576a70', 'eng_second.cpp', 'thiscall, ret 0',
          'como a da First (6º, 3º, 5º e 4º), com +0x3a = o extra da virtual +0x3c (linhas 0x2b6 e 0x2cc)',
          'like the First one (6th, 3rd, 5th and 4th), with +0x3a = the extra from virtual +0x3c (lines 0x2b6 and 0x2cc)'),
         ('mata_mata_da_third', 'EngThird::create_play_off_stage', '0x5783c0', 'eng_third.cpp', 'thiscall, ret 0',
          'como a da Second, com o 7º, o 4º, o 6º e o 5º (chaves 7º × 4º e 6º × 5º; linhas 0x2be e 0x2d5)',
          'like the Second one, with the 7th, 4th, 6th and 5th (ties 7th v 4th and 6th v 5th; lines 0x2be and 0x2d5)')])
CAMPOS = [('liga +0xb1: a tabela (linhas de 0x3b bytes, o clube no +0); ponteiro; lê (mata_mata_da_first, mata_mata_da_second, mata_mata_da_third)',
           'league +0xb1: the table (0x3b-byte rows, club at +0); pointer; reads (mata_mata_da_first, mata_mata_da_second, mata_mata_da_third)'),
          ('liga +0xbf: quantos vão ao mata-mata; valor; lê (as três)',
           'league +0xbf: how many go to the play-offs; value; reads (all three)'),
          ('liga +0xc: as vagas das fases; ponteiro; grava a primeira (as três)',
           'league +0xc: the stage slots; pointer; writes the first one (all three)'),
          ('liga +4 / +0x40: o registro da competição e a palavra passada à fase; ponteiro e valor; lê (as três)',
           'league +4 / +0x40: the competition record and the word passed to the stage; pointer and value; reads (all three)'),
          ('fase (Cup_Stage) +0xa7 / +0x3e: a lista dos clubes (6 bytes) e a conta; ponteiro e valor; grava pelo construtor (as três)',
           'stage (Cup_Stage) +0xa7 / +0x3e: the club list (6 bytes) and the count; pointer and value; written by the constructor (all three)')]
INTERESSA = []
