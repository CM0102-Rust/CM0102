# Mutacoes (r7_04_1). TELAS: a cadeia nao roda; o texto nao volta para a simulacao, e o comparador do escritor
# (compare_text_writer: o texto, os vinculos e o gerador) as acusa quando a noticia e aberta jogando comparando.
# Conferidas na emulacao (unicorn, a original e a copia no mesmo mundo, r7_04\check_r7_04.py): as tres mudam o rastro.
# - os vinculos do treinador com o codigo do clube (todo corpo cujo treinador ainda confere);
# - o paragrafo das caracteristicas sem a condicao do "cedo demais" do veredito (todo corpo com caracteristicas e
#   veredito cedo demais, potencial bom ou grande);
# - o jogador "ainda se adaptando" lido como "adaptado" (todo corpo de jogador ainda se adaptando).
MUTACOES = [
    ('r7_04_1_vinculo_do_treinador',
     "        exe::text_add_link(out, t.coach.as_ptr(), 0x7d7, p); // r7_04_1: the coach's links",
     "        exe::text_add_link(out, t.coach.as_ptr(), 0x7d5, p); // r7_04_1 mutation",
     'r7_04_1 report body'),
    ('r7_04_1_caracteristicas_cedo_demais',
     '            && !self.has_verdict(COACH_VERDICT_TOO_SOON) // r7_04_1: not too soon',
     '            // r7_04_1 mutation',
     'r7_04_1 traits paragraph'),
    ('r7_04_1_adaptacao',
     '        } else if self.is_settled(COACH_SETTLED_SETTLING) {',
     '        } else if self.is_settled(COACH_SETTLED_SETTLED) {',
     'r7_04_1 report body'),
]
