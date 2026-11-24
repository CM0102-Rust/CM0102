# A nova edicao da Copa das Confederacoes (fifa_confed.cpp; leitor cx06, rodada 5, 24/11/2026). Simulacao com as 8 ligas
# e ATE 2004-11-01 (o mesmo do cx06_11..13 e da Copa da Asia do cx05): a copa de 2003 termina em 09/08/2003 e a nova
# edicao (2005) vem na virada seguinte [inferencia: a data exata nao foi vista]; 0 chamadas na cobertura do mundo ate
# 2002-08-01.
# - 0x58bff0 FifaConfed::start_new_season thiscall() ret 0 (virtual 8): +0x4c 0, League::free_table 0x687970 (push 0),
#   free(+0xba) e zera, Comp_Stats::archive_season_and_reset 0x4a1c50 (push 1) se houver, as fases ate a atual (virtual 0
#   com 1; o vetor fica), o ano + 2, +0x30 -1, +0x45 0 e set_up_rules (virtual 0x8c, ja em Rust: 0x58bdb0), cujo eax
#   devolve. Prologo push esi; mov esi, ecx; push edi; push 0 (6 bytes).
# Sem mutacao propria: o que a nova edicao monta (a data do sorteio de 2005) so e usado depois da data da cadeia; o molde
# e o das outras selecoes ja provadas (a ordem: arquivar antes de desmontar as fases, como a do Mundial de Clubes).
# Iguais no cm0102-1080.exe.
# Comparador: so a original (libera as fases), com o plano (o ano + 2, +0x30 -1, +0x45 0).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('fifa_confed_start_new_season', 0x58bff0, [0x56, 0x8b, 0xf1, 0x57, 0x6a, 0x00])]
DOCS = ('a nova edição da Copa das Confederações', "the Confederations Cup's new edition",
        [('fifa_confed_start_new_season', 'FifaConfed::start_new_season', '0x58bff0', 'fifa_confed.cpp', 'thiscall, ret 0',
          'começa a edição seguinte da Copa das Confederações, dois anos depois: limpa a tabela e as datas, arquiva as estatísticas, desmonta as fases e refaz as regras',
          'starts the next Confederations Cup, two years later: clears the table and the dates, archives the statistics, destroys the stages and sets up the rules again')])
CAMPOS = []
INTERESSA = []
