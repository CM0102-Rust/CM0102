# Mutacoes (sx01d_1). A rotina roda em toda resposta a uma historia de time (44 chamadas na carreira ENG ate
# 2002-06-01); o comparador so roda a original, entao a prova e a carreira:
# - a base x = trunc(stat x 30) em vez de x 67 (muda as confiancas e os efeitos de quase todos os tipos);
# - o peso dos efeitos (a stat) dobrado (muda o humor do elenco em todos os ramos);
# - a resposta 2 do molde comum passa as marcas 4 em vez de 8 aos efeitos (humor do elenco);
# - a torcida da resposta 3 dos tipos 0x3c/0x3d com a soma dos sorteios.
MUTACOES = [
    ('sx01d_base_x30',
     'let base = ftol(stat as f64 * read_f32(TEAM_STORY_F32_67) as f64);',
     'let base = ftol(stat as f64 * read_f32(STORY_F32_30) as f64); // mutation: x 30',
     'sx01d_1 team story standard 2'),
    ('sx01d_peso_dos_efeitos',
     'f(self.this, self.club.addr(), kind, flags, self.stat.to_bits(), self.response, change, player)',
     'f(self.this, self.club.addr(), kind, flags, (self.stat * 2.0).to_bits(), self.response, change, player) // mutation',
     'sx01d_1 team story standard 2'),
    ('sx01d_resposta_2_marcas',
     's.effects(kind, 8, ftol(team_story_at_most(x * read_f64(TEAM_STORY_F64_NEG_0_25)',
     's.effects(kind, 4 /* mutation */, ftol(team_story_at_most(x * read_f64(TEAM_STORY_F64_NEG_0_25)',
     'sx01d_1 team story standard 2'),
    ('sx01d_torcida_soma',
     'let d = exe::utils_random(0x14).wrapping_sub(exe::utils_random(0x14));',
     'let d = exe::utils_random(0x14).wrapping_add(exe::utils_random(0x14)); // mutation: the sum',
     'sx01d_1 team story crowd 3'),
]
