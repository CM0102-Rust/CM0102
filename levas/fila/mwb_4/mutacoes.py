# Mutacoes (mwb_4): cada copa pula um ano na virada (o ano da edicao nova errado: as datas das rodadas um ano depois).
# A Copa Challenge joga em julho (a carreira ve); as outras duas, so o comparador (ano +2 em vez de +1).
MUTACOES = [
    ('mwb4_challenge_pula_um_ano',
     '[0x180, 0x187, 0x18e], 1, "mwb_4 challenge cup turnover") // mwb4: the next Challenge Cup',
     '[0x180, 0x187, 0x18e], 2, "mwb_4 challenge cup turnover") // mutation: two years on',
     'mwb_4 challenge cup turnover'),
    ('mwb4_copa_da_escocia_pula_um_ano',
     '[0x20f, 0x216, 0x21d], 1, "mwb_4 scottish cup turnover") // mwb4: the next Scottish Cup',
     '[0x20f, 0x216, 0x21d], 2, "mwb_4 scottish cup turnover") // mutation: two years on',
     'mwb_4 scottish cup turnover'),
    ('mwb4_copa_de_gales_pula_um_ano',
     '[0x17b, 0x182, 0x189], 1, "mwb_4 welsh cup turnover") // mwb4: the next Welsh Cup',
     '[0x17b, 0x182, 0x189], 2, "mwb_4 welsh cup turnover") // mutation: two years on',
     'mwb_4 welsh cup turnover'),
]
