# Os construtores de três torneios de seleções: a Copa da Ásia, o futebol olímpico e a Copa das Confederações
# (asia_nations.cpp, olympics.cpp, fifa_confed.cpp). Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x40e860 (AsiaNations::AsiaNations), 0x79f530 (Olympics::Olympics), 0x58b310 (FifaConfed::FifaConfed).
# Os dois primeiros têm o corpo da Copa Africana (build_nations_tournament, já em Rust), com a edição no ciclo de
# quatro anos a partir de 1996 (2004 num jogo de 2001); a Copa das Confederações tem corpo próprio: ano ímpar diferente
# de 2001 (2003), 2 fases, as estatísticas ANTES das regras (sem estatísticas, a caixa da linha 0x53 e nada de regras;
# o retorno das regras não é lido) e nenhum +0xd9 no construtor (as regras gravam 0x28).
# Convenções conferidas: os construtores são thiscall, ret 8; dentro: 0x682200 (mov ecx -> thiscall, ret 0), 0x687b10
# (push 1; mov ecx -> thiscall, ret 4), 0x944e46/0x944cf1 (push; add esp -> cdecl), 0x49ee70 (push this; mov ecx ->
# thiscall, ret 4) e as regras (mov ecx -> thiscall, ret 0).
# Comparadores: só a original (alocam fases e estatísticas), com o plano: this, vtable, registro, ano da edição e +0x51.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construct_asia_nations', 0x40e860, [0x6a, 0xff, 0x68, 0xa9, 0x38, 0x95, 0x00]),
           ('construct_olympics', 0x79f530, [0x6a, 0xff, 0x68, 0xb9, 0xef, 0x95, 0x00]),
           ('construct_fifa_confed', 0x58b310, [0x6a, 0xff, 0x68, 0x69, 0x8d, 0x95, 0x00])]
DOCS = ('os construtores da Copa da Ásia, do futebol olímpico e da Copa das Confederações',
        'the constructors of the Asian Cup, the Olympic football and the Confederations Cup',
        [('construct_asia_nations', 'AsiaNations::AsiaNations', '0x40e860', 'asia_nations.cpp', 'thiscall, ret 8', 'regra: a Copa da Ásia é de quatro em quatro anos a partir de 1996 (2000, 2004...); a edição do jogo é a do ano do jogo levada ao ciclo (num jogo de 2001, a de 2004); 3 fases', "rule: the Asian Cup is every four years from 1996 (2000, 2004...); the game's edition is the game's year moved to the cycle (in a 2001 game, 2004); 3 stages"),
         ('construct_olympics', 'Olympics::Olympics', '0x79f530', 'olympics.cpp', 'thiscall, ret 8', 'regra: o futebol olímpico é de quatro em quatro anos a partir de 1996; a edição do jogo é o ano olímpico seguinte (num jogo de 2001, 2004); 4 fases', "rule: the Olympic football is every four years from 1996; the game's edition is the next Olympic year (in a 2001 game, 2004); 4 stages"),
         ('construct_fifa_confed', 'FifaConfed::FifaConfed', '0x58b310', 'fifa_confed.cpp', 'thiscall, ret 8', 'regra: a Copa das Confederações é nos anos ímpares, menos 2001 (num jogo de 2001, a de 2003); 2 fases; as estatísticas vêm antes das regras', 'rule: the Confederations Cup is in odd years other than 2001 (in a 2001 game, 2003); 2 stages; the statistics come before the rules')])
CAMPOS = [('competição +0x40 ano da edição (word), +4 registro, +0xc/+0x2c fases, +8 estatísticas, +0x51, +0x45, +0x44, +0x30; valor/ponteiro; grava', 'competition +0x40 edition year (word), +4 record, +0xc/+0x2c stages, +8 statistics, +0x51, +0x45, +0x44, +0x30; value/pointer; writes')]
