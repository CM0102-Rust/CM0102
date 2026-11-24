# O construtor e a virada do FA Trophy (eng_fa_trophy.cpp; leitor sbF, rodada 5, 24/11/2026). Carreira ENG com a base
# NORMAL (ver sbF_9: o FA Trophy so existe com a Conference carregada; BASE='NORMAL', que nenhuma ferramenta le ainda).
# - 0x570c00 EngFaTrophy::EngFaTrophy thiscall(ano, registro) ret 8: construct_irish_cup (conferido instrucao a
#   instrucao contra a NirCup::NirCup) com a vtable 0x969bc8, a disciplina ENGLAND, EngFaTrophy::select_teams 0x5712b0,
#   o premio 300.000 e as caixas 0x4b, 0x59 e 0x60.
# - 0x5714a0 EngFaTrophy::start_new_season thiscall() ret 0: start_new_cup_season (conferido contra a
#   NirCup::start_new_season) com 0x5712b0 e as caixas 0x1b3, 0x1ba e 0x1c1.
# Prologos: push -1; push 0x958309 (7 bytes); sub esp, 0x200 (6 bytes). Iguais no cm0102-1080.exe.
# Comparadores: so a original, com o plano (compare_irish_cup_constructor, compare_cup_new_season).
# Integrar junto com o sbF_9 (o construtor chama EngFaTrophy::select_teams pelo endereco).
SAVE = False
BASE = 'NORMAL'
ROTINAS = [('construct_eng_fa_trophy', 0x570c00, [0x6a, 0xff, 0x68, 0x09, 0x83, 0x95, 0x00]),
           ('eng_fa_trophy_start_new_season', 0x5714a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o construtor e a virada do FA Trophy',
        'the FA Trophy constructor and turnover',
        [('construct_eng_fa_trophy', 'EngFaTrophy::EngFaTrophy', '0x570c00', 'eng_fa_trophy.cpp', 'thiscall, ret 8',
          'regra: o FA Trophy é uma copa dos clubes de fora da liga de uma nação no ano do jogo, com prêmio de 300.000, 3 reservas e 3 substituições, sob a disciplina inglesa; sem estado gravado, os 32 inscritos, as rodadas, a árvore de confrontos, as estatísticas e a classificação final',
          'rule: the FA Trophy is a cup of one nation\'s non-league clubs of the game\'s year, with a prize fund of 300,000, 3 substitutes on the bench and 3 substitutions, under the English discipline rules; with nothing saved, its 32 entrants, the rounds, the tie tree, the statistics and the final ranking'),
         ('eng_fa_trophy_start_new_season', 'EngFaTrophy::start_new_season', '0x5714a0', 'eng_fa_trophy.cpp', 'thiscall, ret 0',
          'regra: na virada o FA Trophy é refeito: as tabelas da edição anterior liberadas, as estatísticas arquivadas, o ano +1 e os inscritos, as regras, a árvore de confrontos e as posições previstas de novo',
          'rule: at the season turn the FA Trophy is rebuilt: last edition\'s tables freed, the statistics archived, the year +1 and the entrants, the rules, the tie tree and the predicted positions again')])
CAMPOS = []
INTERESSA = []
