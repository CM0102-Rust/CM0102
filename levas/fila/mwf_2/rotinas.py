# As viradas de temporada de Copa da Coreia, no molde das copas (bra_cup_start_new_season / start_new_cup_season, já em Rust desde a 282a: só mudam o arquivo
# das caixas, as linhas e a rotina dos participantes; conferido instrução por instrução pelo norm.py do leitor mwf).
# Leitor mwf (12/11/2026), rodada 2 (o mundo inteiro até 2002-08-01): cada rotina roda 1 vez (a virada de meados de
# 2002; docs/cobertura_mundo.txt).
# Convenções: virtual +0x08 do objeto (mov ecx, esi; call [eax+8] no RulingBody / na liga que vira) -> thiscall, ret 0.
# Comparador: só a original (libera e refaz a copa), com o ano +1 (compare_cup_new_season).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('kor_fa_cup_start_new_season', 0x66acc0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada da Copa da Coreia',
        'the season turnover of the Korean FA Cup',
        [('kor_fa_cup_start_new_season', 'KorFaCup::start_new_season', '0x66acc0', 'kor_fa_cup.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Copa da Coreia refaz a copa: as tabelas da edição anterior liberadas, a temporada arquivada nas estatísticas, o ano mais um, os participantes (KorFaCup::collect_entrants), as rodadas e a árvore de confrontos; as posições previstas', "rule: at the season turnover the Korean FA Cup is rebuilt: last edition's tables freed, the season archived in the statistics, the year plus one, the entrants (KorFaCup::collect_entrants), the rounds and the tie tree; then the predicted positions")])
CAMPOS = [('copa +0x4c, +0xa3, +0xa7, +0xab, +0xad, +0xb1, +0x40 (a virada); valor/ponteiro; grava', 'cup +0x4c, +0xa3, +0xa7, +0xab, +0xad, +0xb1, +0x40 (the turnover); value/pointer; writes')]
