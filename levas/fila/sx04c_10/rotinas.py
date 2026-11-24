# Os tecnicos do mes da Polonia (award/poland_awards.cpp; leitor sx04c, rodada 5, 23/11/2026). Usa nation_award e
# award_month_manager (sx03_2): integrar depois dele.
# NUNCA RODA: o construtor da Polonia (0x7cd130) nao cria notas do mes e o NationAwards::daily_update (0x768310) sai
# sem chamar a virtual +0xc quando +0xa e nulo (defeito do original: os tecnicos do mes poloneses nunca sao dados).
# Entra pelo "exe inteiro" (Lucas, 23/11); sem mutacoes.
# Convencao conferida: 0x7cd790 thiscall, ret 0: 0x86f950 + 0x7671c0 nos premios 4 e 9 (this so para a tabela).
# Comparador: so a original.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ROTINAS = [('poland_awards_on_month_change', 0x7cd790, [0xa1, 0x58, 0xf8, 0x9c, 0x00])]
DOCS = ('os técnicos do mês da Polônia',
        'the Polish managers of the month',
        [('poland_awards_on_month_change', 'PolandAwards::on_month_change', '0x7cd790', 'award/poland_awards.cpp', 'thiscall',
          'na virada do mês, o técnico do mês de cada divisão; nunca roda, porque a Polônia não tem notas do mês (defeito do original)',
          'at the turn of the month, the manager of the month of each division; never runs, because Poland has no month ratings (defect of the original)')])
CAMPOS = []
INTERESSA = []
