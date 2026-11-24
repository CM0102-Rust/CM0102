# Mutacoes (sbF_7), carreira com a Holanda ate 2002-08-01. Nenhuma abre caixa.
# - os potes de sete e nao de oito: o embaralhamento muda os grupos de agosto (a carreira acusa; o comparador roda so a
#   original);
# - os clubes da Copa UEFA depois dos da Liga dos Campeoes: quem fica com a vaga f2-7 (o primeiro sorteado do
#   mata-mata) e a ordem dos que entram na 3a rodada mudam (a carreira acusa).
# O dia de montagem (update_for_date) nao tem mutacao propria: o comparador confere o plano (ativo, a lista e as 19
# fases) e a montagem passa pelas mutacoes acima; mudar a condicao deixaria a copa sem fases (o jogo cai).
MUTACOES = [
    ('sbF_7_potes_de_sete',
     'exe::comp_util_shuffle_pointer_array(list + 0x20 * pot, 8); // sbF7: the pots of eight',
     'exe::comp_util_shuffle_pointer_array(list + 0x20 * pot, 7); // mutation',
     'sbF_7 dutch cup entrants'),
    ('sbF_7_europeus_trocados',
     '    for key in [KEY_CLUB_COMP_INTER_TOTO_CUP, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE] {\n        append_dutch_cup_entrants(',
     '    for key in [KEY_CLUB_COMP_INTER_TOTO_CUP, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP] { // mutation\n        append_dutch_cup_entrants(',
     'sbF_7 dutch cup entrants'),
]
