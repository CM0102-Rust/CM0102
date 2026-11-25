# Mutacoes (sbI_2), carreira ENG ate 2002-06-01:
# - o segundo dia de planejar a pre-temporada 7 dias depois (10 de janeiro): o planejamento dos amistosos de janeiro de
#   2002 cai noutro dia;
# - a tabela de sedes com 56 edicoes: um sorteio a menos na criacao (a semente da carreira toda muda) e a ultima edicao
#   fora;
# - o weather.cfg lido duas vezes: os sorteios do tempo de cada cidade de novo (a semente muda).
MUTACOES = [
    ('sbI_2_amistosos_dia_b',
     '    exe::date_date_549f70(&mut january, 3, 0, 0x7ce, u32::MAX); // 3 January 1998',
     '    exe::date_date_549f70(&mut january, 0xa, 0, 0x7ce, u32::MAX); // mutation',
     'sbI_2 new friendlies'),
    ('sbI_2_sedes_56',
     '    write_i16(this + 4, 0x39); // +4: the entry count, 57',
     '    write_i16(this + 4, 0x38); // mutation',
     'sbI_2 new host table'),
    ('sbI_2_tempo_duas_vezes',
     '    load_config(this); // sbI_2: the weather.cfg',
     '    load_config(this);\n    load_config(this); // mutation',
     'sbI_2 new weather'),
]
