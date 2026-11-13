# Mutacoes (mwb_18): o primeiro grupo com os clubes uma linha adiante (outra ordem, outros jogos); o saldo antes do
# confronto direto no desempate dos grupos (outro classificado quando ha empate). A passagem de fase fica sem mutacao:
# sem o mata-mata a copa cai (a sexta vaga nula nas honrarias); so o comparador a ve.
MUTACOES = [
    ('mwb18_primeiro_grupo_linhas_giradas', 'welsh_cup_group_table(League(this), 3, 6, 0xad56f4, 0x182, 0, "mwb_18 welsh league cup table") // mwb18: the League Cup first group', 'welsh_cup_group_table(League(this), 3, 6, 0xad56f4, 0x182, 1, "mwb_18 welsh league cup table") // mutation: rotated rows', 'mwb_18 welsh league cup table'),
    ('mwb18_copa_da_liga_dois_pontos', 'welsh_cup_rules(1, [4, 1, 2]) // mwb18: the League Cup group rules', 'welsh_cup_rules(1, [1, 4, 2]) // mutation: goal difference first', 'mwb_18 welsh league cup rules'),
]
