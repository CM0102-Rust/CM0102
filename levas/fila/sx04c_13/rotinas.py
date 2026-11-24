# Os jogadores do mes da Russia (award/russia_awards.cpp; leitor sx04c, rodada 5, 23/11/2026). Usa nation_award
# (sx03_2) e award_month_best_player (sx04c_1): integrar depois deles.
# NUNCA RODA: o construtor da Russia (0x7ec210) nao cria notas do mes e o NationAwards::daily_update sai sem chamar a
# virtual +0xc quando +0xa e nulo (defeito do original: os jogadores do mes russos nunca sao dados; se rodasse sem
# notas, a leitura do mes em [+0xa]+0xc derrubaria o jogo). Entra pelo "exe inteiro"; sem mutacoes.
# Convencao conferida: 0x7ec6c0 thiscall, ret 0: sai com o mes das notas (+0xc) = 5; senao, por divisao,
# MONTH_RATINGS::find_best_player 0x768130 thiscall([chave], -1) ret 8 e, com resultado, MONTH_AWARD::set_winner
# 0x7671c0 (id, id do clube de [pessoa+0x39] ou 0, mes +0xc, ano +0xd das notas) nos premios 3 e 6.
# Comparador: so a original.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ROTINAS = [('russia_awards_on_month_change', 0x7ec6c0, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x4e, 0x0a])]
DOCS = ('os jogadores do mês da Rússia',
        'the Russian players of the month',
        [('russia_awards_on_month_change', 'RussiaAwards::on_month_change', '0x7ec6c0', 'award/russia_awards.cpp', 'thiscall',
          'na virada do mês (exceto junho), o jogador do mês de cada divisão, o melhor das notas do mês, com o clube dele; nunca roda, porque a Rússia não tem notas do mês (defeito do original)',
          'at the turn of the month (except June), the player of the month of each division, the best of the month ratings, with his club; never runs, because Russia has no month ratings (defect of the original)')])
CAMPOS = []
INTERESSA = []
