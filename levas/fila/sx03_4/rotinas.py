# Os premios internacionais, as notas do mes so das ligas e as regras da Dinamarca (leitor sx03, rodada 5,
# 23/11/2026). Rodam na carreira ENG: a diaria internacional todo dia (virtual +4 da vtable 0x96bb78 do
# InternationalAwards; da nada em 2001 e da o jogador do ano mundial em 6 de maio de 2002; 6 e 16 de junho ficam
# depois de 2002-06-01); 0x5a96b0 e a virtual +8 da vtable da Inglaterra (0x969f38) e da Finlandia, a cada partida;
# o construtor da Dinamarca uma vez na montagem das regras (0x8d2892, sem condicao).
# Convencoes conferidas: 0x7683c0 thiscall 10 args ret 0x28; 0x768b20 thiscall(hoje, premio) ret 8; 0x768720
# thiscall 7 args ret 0x1c; 0x7683a0 thiscall(partida, registro) ret 8; 0x4b0700 cdecl(id) (Rust); 0x555460 thiscall
# 5 args ret 0x14.
# Usa os auxiliares do sx03_2 (nation_award, awards_day_is): integrar depois dele.
# Comparadores: a diaria e as notas so a original (alocam, noticia, reputacao); a Dinamarca por
# compare_transfer_rules_ctor.
SAVE = False
ROTINAS = [
    ('international_awards_daily_update', 0x632e20, [0x83, 0xec, 0x10, 0x53, 0x56]),
    ('nation_awards_record_league_match_ratings', 0x5a96b0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
    ('construct_denmark_rules', 0x555460, [0x8b, 0x44, 0x24, 0x14, 0x8b, 0x54, 0x24, 0x0c]),
]
DOCS = ('os prêmios internacionais, as notas do mês só das ligas e as regras de transferência da Dinamarca',
        "the international awards, the month ratings of league matches only and Denmark's transfer rules",
        [('international_awards_daily_update', 'international_awards_daily_update', '0x632e20', 'international_awards.cpp', 'thiscall, ret 4',
          'regra: nada em 2001; em 6 de maio, o jogador do ano mundial (prêmio 0) pelas notas do ano mundiais do gestor, filtro 2; em 6 de junho, o prêmio 0 se decide pela lista de indicados e o prêmio 1 sai das mesmas notas, filtro 3; em 16 de junho, os três melhores da África, Europa, América do Sul e Oceania (prêmios 2 a 5); sem a conferência da virada do mês da base',
          'rule: nothing in 2001; on 6 May, the world player of the year (award 0) from the manager\'s world year ratings, filter 2; on 6 June, award 0 is decided from its shortlist and award 1 comes from the same ratings, filter 3; on 16 June, the best three of Africa, Europe, South America and Oceania (awards 2 to 5); no base month change check'),
         ('nation_awards_record_league_match_ratings', 'nation_awards_record_league_match_ratings', '0x5a96b0', 'finland_awards.cpp', 'thiscall, ret 8',
          'regra: a partida entra nas notas do mês só quando a competição é uma divisão de liga (Inglaterra e Finlândia)',
          'rule: the match goes into the month ratings only when its competition is a league division (England and Finland)'),
         ('construct_denmark_rules', 'construct_denmark_rules', '0x555460', 'denmark_awards.cpp', 'thiscall, ret 0x14',
          'regra: a base das regras, a vtable da Dinamarca (0x969860) e nenhuma janela de transferências (contagem e tabela zeradas): os clubes dinamarqueses contratam a qualquer hora',
          "rule: the base rules, Denmark's vtable (0x969860) and no transfer window (count and table cleared): Danish clubs sign at any time")])
CAMPOS = []
INTERESSA = []
