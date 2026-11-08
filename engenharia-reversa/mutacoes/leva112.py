# Mutacoes da leva112 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva112.py --jogo 2001-08-20
MUTACOES = [
    ('m_maior', "            if melhor == -1 || v > nota {", "            if melhor == -1 || v >= nota {"),
    ('m_ativo', "        if byte(p, match_player::POSITION) >= 0 {\n            let v = exe::match_man_get_captain_score(0, p, club);", "        if byte(p, match_player::POSITION) >= -1 {\n            let v = exe::match_man_get_captain_score(0, p, club);"),
]
