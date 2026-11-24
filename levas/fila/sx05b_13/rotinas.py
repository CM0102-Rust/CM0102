# As equipes da semana da Suecia e as regras de transferencia suecas (award/sweden_awards.cpp; leitor sx05b, rodada
# 5, 23/11/2026). Usa nation_awards_team_of_the_week_of, nation_award e
# build_team_of_the_week (sx05b_5 e sx03_2) e compare_transfer_rules_ctor (england_rules.rs): integrar depois do
# sx05b_5 e do sx03_2.
# A equipe da semana e chamada pela virtual das equipes de premio (NationAwards); as regras pelo setup_transfer_rules
# (0x8d2d05, na criacao do jogo, so com a Suecia). LIGAS = 'ENG SWE FIN NOR POR'.
# Convencoes conferidas na desmontagem:
# - 0x893d60 thiscall(premio) ret 4: compara com [[+4]+0x1c] e [[+4]+0x24] (premios 7 e 9);
#   NationAwards::update_team_of_the_week 0x769c70 thiscall([[premio+9]], premio) ret 8 (o ecx e o this);
# - 0x893d80 thiscall(a1..a5) ret 0x14: TRANSFER_RULES::TRANSFER_RULES 0x8e84a0 (a1..a5) ret 0x14; [this] = 0x970368,
#   byte +8 = 0, dword +4 = 0, dword +0x15 = 1; devolve this.
# Comparadores: a equipe da semana so a original (a selecao nao se refaz); as regras lado a lado num rascunho
# (compare_transfer_rules_ctor, como as outras regras de transferencia).
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ROTINAS = [('sweden_awards_team_of_the_week', 0x893d60, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('construct_sweden_rules', 0x893d80, [0x8b, 0x44, 0x24, 0x14, 0x8b, 0x54, 0x24, 0x0c])]
DOCS = ('as equipes da semana suecas e as regras de transferência da Suécia',
        'the Swedish teams of the week and the Swedish transfer rules',
        [('sweden_awards_team_of_the_week', 'SwedenAwards::update_team_of_the_week', '0x893d60', 'award/sweden_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana de um prêmio sueco: só para as equipes da semana da Premier Division e da First Division (prêmios 7 e 9), pela competição de clubes do prêmio',
          "builds the team of the week of a Swedish award: only for the Premier Division and First Division teams of the week (awards 7 and 9), by the award's club competition"),
         ('construct_sweden_rules', 'SWEDEN_RULES::SWEDEN_RULES', '0x893d80', 'award/sweden_awards.cpp', 'thiscall, ret 0x14',
          'monta as regras de transferência da Suécia: a base, a tabela virtual da classe, nenhuma janela de transferências e a marca +0x15 = 1 (o contrato de tipo 5 não tira a elegibilidade)',
          'builds the Swedish transfer rules: the base, the class vtable, no transfer windows and the +0x15 flag = 1 (a type 5 contract does not make a player ineligible)')])
CAMPOS = []
INTERESSA = []
