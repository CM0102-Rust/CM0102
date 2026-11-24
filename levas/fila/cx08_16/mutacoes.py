# Mutacoes (cx08_16), com a Belgica ate 2002-08-01 (a carreira acusa: mudam os jogos e quem vai a repescagem):
# - o grupo da repescagem joga em turno unico;
# - as fases dos periodos sao em ida e volta;
# - o campeao do periodo fica com o status 2 em vez de 1 na tabela da liga.
MUTACOES = [
    ('cx08_16_grupo_turno_unico',
     'ctor(p, this.addr(), 4, list, 2, this.competition().addr(), table,',
     'ctor(p, this.addr(), 4, list, 1 /* mutation */, this.competition().addr(), table,',
     'cx08_16 belgian play-off group'),
    ('cx08_16_periodo_ida_e_volta',
     'ctor(p, this.addr(), this.club_count() as u16 as u32, list, 1, this.competition().addr(), 0, 0,',
     'ctor(p, this.addr(), this.club_count() as u16 as u32, list, 2 /* mutation */, this.competition().addr(), 0, 0,',
     'cx08_16 belgian period stage'),
    ('cx08_16_campeao_status_2',
     '        if row.club() as usize == club && row.status() == 0xff {\n            row.set_status(1);',
     '        if row.club() as usize == club && row.status() == 0xff {\n            row.set_status(2); // mutation',
     'cx08_16 belgian period winner'),
]
