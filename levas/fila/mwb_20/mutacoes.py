# Mutacoes (mwb_20): o primeiro grupo com os clubes uma linha adiante (outra ordem, outros jogos); o confronto direto
# antes do saldo no desempate. A passagem de fase fica sem mutacao (como a do mwb_18).
MUTACOES = [
    ('mwb20_primeiro_grupo_linhas_giradas', 'welsh_cup_group_table(League(this), 4, 2, 0xad572c, 0x181, 0, "mwb_20 welsh premier cup table") // mwb20: the Premier Cup first group', 'welsh_cup_group_table(League(this), 4, 2, 0xad572c, 0x181, 1, "mwb_20 welsh premier cup table") // mutation: rotated rows', 'mwb_20 welsh premier cup table'),
    ('mwb20_premier_confronto_direto_primeiro', 'welsh_cup_rules(2, [1, 2, 4]) // mwb20: the Premier Cup group rules', 'welsh_cup_rules(2, [4, 1, 2]) // mutation: head-to-head first', 'mwb_20 welsh premier cup rules'),
]
