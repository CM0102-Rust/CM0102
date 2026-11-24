# Os tres orgaos disciplinares gerais, construidos em todo jogo (Discipline::create_rule_sets, sempre): o europeu, o
# internacional (FIFA) e o asiatico (rb_europe.cpp, rb_international.cpp, rb_asia.cpp; leitor sx18, rodada 5,
# 24/11/2026). Usa RulingBodyClass, ruling_body_class_construct e compare_ruling_body_class_built do sx18_1: integrar
# depois dele. Rodam na criacao do jogo da carreira ENG.
# Convencoes conferidas na desmontagem: os tres thiscall(arquivo) ret 4: 0x7e7760 (base, construct_ruling_body em
# Rust), depois os campos: europeu vtable 0x96eb34, +4 = 2, +5 = 1, +6 = 0xa, +0x17 = -1, +0x1b Europa, +0x1f/+0x20 =
# 0xff, +0x21 = 1; internacional 0x96ec34, +4 = 3, +5 = 0, +6 = 0xa, +0x17/+0x1b = -1, +0x1f/+0x20/+0x21 = 0xff;
# asiatico 0x96ea14, +4 = 2, +5 = 0x29, +6 = 0xa, +0x17 = -1, +0x1b Asia, +0x1f/+0x20 = 0xff, +0x21 = 0x29; +0x22 = 0
# nos tres. O asiatico recebe arquivo 0 com dados anteriores a versao 0x3e044 (decisao do chamador).
# Comparadores: so a original roda (constroi uma vez) e os campos sao conferidos contra a classe.
SAVE = False
ROTINAS = [('rb_europe_construct', 0x7dd400, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_international_construct', 0x7df180, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_asia_construct', 0x7db360, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('os três órgãos disciplinares gerais: o europeu, o internacional e o asiático',
        'the three general ruling bodies: the European, the international and the Asian',
        [('rb_europe_construct', 'RbEurope::RbEurope', '0x7dd400', 'ruling_body/rb_europe.cpp', 'thiscall, ret 4',
          'monta o órgão europeu sobre a base: tabela virtual 0x96eb34, a vaga 2 do registro, a regra 1, a escala 10, sem nação, a Europa e ele mesmo como regra da confederação',
          'builds the European body on the base: vtable 0x96eb34, record slot 2, rule set 1, scale 10, no nation, Europe and itself as the confederation rule'),
         ('rb_international_construct', 'RbInternational::RbInternational', '0x7df180', 'ruling_body/rb_international.cpp', 'thiscall, ret 4',
          'monta o órgão internacional (FIFA) sobre a base: tabela virtual 0x96ec34, a vaga 3 do registro, a regra 0, a escala 10, sem nação nem continente',
          'builds the international (FIFA) body on the base: vtable 0x96ec34, record slot 3, rule set 0, scale 10, no nation nor continent'),
         ('rb_asia_construct', 'RbAsia::RbAsia', '0x7db360', 'ruling_body/rb_asia.cpp', 'thiscall, ret 4',
          'monta o órgão asiático sobre a base: tabela virtual 0x96ea14, a vaga 2 do registro, a regra 0x29, a escala 10, sem nação, a Ásia e ele mesmo como regra da confederação',
          'builds the Asian body on the base: vtable 0x96ea14, record slot 2, rule set 0x29, scale 10, no nation, Asia and itself as the confederation rule')])
CAMPOS = []
INTERESSA = []
