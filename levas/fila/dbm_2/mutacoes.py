# Mutacoes do estadio em ingles, do .lng do idioma e de soltar as tabelas em ingles. O estadio e a liberacao
# ESCAPAM da prova do save e da carreira (caminho do eng.lng so com outro idioma): so o autoteste_dbm_2
# (CM_AUTOTESTE=1 ou mutacoes.py --autoteste-banco) as acusa, se estiver ligado em desvios::autotestes. A do .lng
# (por.lng -> eng.lng) tambem: o autoteste compara o caminho montado; a carga so pede o idioma das preferencias (0).
MUTACOES = [
    ('dbm2_estadio', '.wrapping_mul(52)) // estádio na tabela', '.wrapping_mul(51)) // estádio na tabela'),
    ('dbm2_lng', '0x9b6314, 0x9b631c, 0x9b6304', '0x9b6314, 0x9b632c, 0x9b6304'),
    ('dbm2_soltar', '(t + 0x20 + 4 * k) as *const usize); // linha da tabela em inglês', '(t + 0x20 + 4 * (k ^ 1)) as *const usize); // linha da tabela em inglês'),
]
