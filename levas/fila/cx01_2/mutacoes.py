# Mutacoes (cx01_2): textos, acusados pela comparacao com CM_TEXTOS=1 (o texto nao volta para a simulacao). Nenhuma abre
# caixa.
# - o corpo do destino sem a ligacao da competicao;
# - a manchete do premio sem a ligacao do clube;
# - a temporada dos direitos de TV com o ano seguinte.
MUTACOES = [
    ('cx01_2_destino_sem_ligacao',
     '        if outcome as u8 == 5 && exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 {\n            return;\n        }',
     '        if outcome as u8 <= 5 { // mutation\n            return;\n        }'),
    ('cx01_2_premio_manchete_sem_ligacao',
     '        lang_sprintf(0xde1f64, 0x99b844, cg, cg, team, money.as_ptr() as usize);\n        exe::text_set_string(out, 0xde1f64 as *const u8);\n        exe::text_add_link(out, team as *const u8, 0x7d5, c);',
     '        lang_sprintf(0xde1f64, 0x99b844, cg, cg, team, money.as_ptr() as usize);\n        exe::text_set_string(out, 0xde1f64 as *const u8); // mutation'),
    ('cx01_2_tv_ano_seguinte',
     '    exe::key_nation_format_club_season(year, c, season.as_mut_ptr() as usize, 0x14);',
     '    exe::key_nation_format_club_season(year + 1, c, season.as_mut_ptr() as usize, 0x14); // mutation'),
]
