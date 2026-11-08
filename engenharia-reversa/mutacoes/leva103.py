# Mutacoes da leva103 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva103.py --jogo 2001-08-20
MUTACOES = [
    ('p_passou', "    let passou = |end: i32| *w(engine::CLOCK) as i32 > byte(m, engine::STOPPAGE_TIME) as i32 + end;", "    let passou = |end: i32| *w(engine::CLOCK) as i32 >= byte(m, engine::STOPPAGE_TIME) as i32 + end;"),
    ('p_intervalo', "            *tb(fixture::HOME_GOALS_HALF_TIME) = byte(m, engine::SCORE);", "            *tb(fixture::HOME_GOALS_HALF_TIME) = byte(m, engine::SCORE_SIDE1);"),
    ('p_20f1', "            fala(call, 0x20f1);", "            fala(call, 0x20f2);"),
    ('p_3de', "            call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, [0x3de, 0, 0, 0, 0]);\n            recuperar(0);", "            call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, [0x3df, 0, 0, 0, 0]);\n            recuperar(0);"),
    ('p_teto0', "            recuperar(0);", "            recuperar(1);"),
    ('p_saida', "        *b(engine::NEXT_KICKOFF_SIDE) = (a == 0) as i8;", "        *b(engine::NEXT_KICKOFF_SIDE) = a;"),
    ('p_placar', "            *tb(fixture::AWAY_GOALS) = byte(m, engine::SCORE_SIDE1);\n            let rule", "            *tb(fixture::AWAY_GOALS) = byte(m, engine::SCORE);\n            let rule"),
    ('p_fim', "                call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, [0, 0, 0, 0, 0]);\n                *w(engine::PERIOD_END) = 0;\n                return 4;", "                call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, [0, 0, 0, 0, 0]);\n                *w(engine::PERIOD_END) = 1;\n                return 4;"),
    ('p_ordem', "                *w(engine::PERIOD_END) = 0;\n                classificar(c);", "                classificar(c);\n                *w(engine::PERIOD_END) = 0;"),
]
