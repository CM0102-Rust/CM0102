# Mutacoes da leva103 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva103.py --jogo 2001-08-20
MUTACOES = [
    ('p_passou', "    let passed = |end: i32| m.clock() as i32 > m.stoppage_time() as i32 + end;", "    let passed = |end: i32| m.clock() as i32 >= m.stoppage_time() as i32 + end;"),
    ('p_intervalo', "            *tb(fixture::HOME_GOALS_HALF_TIME) = m.score();", "            *tb(fixture::HOME_GOALS_HALF_TIME) = m.score_side1();"),
    ('p_20f1', "            fala(call, 0x20f1);", "            fala(call, 0x20f2);"),
    ('p_3de', "            call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m.addr(), [0x3de, 0, 0, 0, 0]);\n            recuperar(0);", "            call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m.addr(), [0x3df, 0, 0, 0, 0]);\n            recuperar(0);"),
    ('p_teto0', "            recuperar(0);", "            recuperar(1);"),
    ('p_saida', "        m.set_next_kickoff_side((a == 0) as i8);", "        m.set_next_kickoff_side(a);"),
    ('p_placar', "            *tb(fixture::AWAY_GOALS) = m.score_side1();\n            let rule", "            *tb(fixture::AWAY_GOALS) = m.score();\n            let rule"),
    ('p_fim', "                call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m.addr(), [0, 0, 0, 0, 0]);\n                m.set_period_end(period::OVER_IN_NORMAL_TIME);\n                return 4;", "                call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m.addr(), [0, 0, 0, 0, 0]);\n                m.set_period_end(1);\n                return 4;"),
    ('p_ordem', "                m.set_period_end(period::OVER_IN_NORMAL_TIME);\n                classificar(c);", "                classificar(c);\n                m.set_period_end(period::OVER_IN_NORMAL_TIME);"),
]
