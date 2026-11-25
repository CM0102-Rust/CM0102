# A virada do mês e a equipe da semana dos prêmios irlandeses (award/ireland_awards.cpp; leitor rx11, rodada 6,
# 25/11/2026). Simulação com a Irlanda carregada (a virtual +0xc a cada mês, a +0x10 a cada semana): LIGAS = 'ENG IRL
# NIR', como o rx11_9. Usa o nation_awards_team_of_the_week_of do sx05b_5: integrar depois dele.
# - 0x63c160 IrelandAwards::on_month_change thiscall ret 0: MONTH_AWARD::award_best_rated_player 0x767150 (prêmio 3, as
#   notas do mês +0xa, -1); para a Premier ([0x9cf80c], prêmio 10) e a Primeira ([0x9cf810], prêmio 17)
#   Staff_Records::find_manager_of_the_month 0x86f950 e, achado, MONTH_AWARD::set_winner 0x7671c0 (o id, o id do clube,
#   o mês +0x14 e o ano +0x16 do Staff_Records). Prólogo push esi; mov esi, ecx; push -1 (5 bytes).
# - 0x63c1f0 IrelandAwards::update_team_of_the_week thiscall(premio) ret 4: o prêmio igual a +4[4] ou +4[11]:
#   0x769c70(id da competição, premio). Prólogo mov edx, [ecx+4]; mov eax, [esp+4] (7 bytes).
# Iguais no cm0102-1080.exe. Comparadores: só a original (os prêmios saem uma vez).
SAVE = False
LIGAS = 'ENG IRL NIR'
ROTINAS = [('ireland_awards_on_month_change', 0x63c160, [0x56, 0x8b, 0xf1, 0x6a, 0xff]),
           ('ireland_awards_update_team_of_the_week', 0x63c1f0, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('a virada do mês e a equipe da semana dos prêmios irlandeses',
        "the Irish awards' month change and team of the week",
        [('ireland_awards_on_month_change', 'IrelandAwards::on_month_change', '0x63c160', 'award/ireland_awards.cpp', 'thiscall, ret 0',
          'regra: a cada virada de mês a Irlanda dá o jogador do mês pelas notas do mês e o técnico do mês da Premier e da Primeira Divisão',
          'rule: at each month change Ireland gives the player of the month from the month ratings and the manager of the month of the Premier and First Divisions'),
         ('ireland_awards_update_team_of_the_week', 'IrelandAwards::update_team_of_the_week', '0x63c1f0', 'award/ireland_awards.cpp', 'thiscall, ret 4',
          'regra: a equipe da semana é montada para o prêmio da equipe da semana da Premier ou da Primeira Divisão irlandesa',
          'rule: the team of the week is built for the team of the week award of the Irish Premier or First Division')])
CAMPOS = []
INTERESSA = []
