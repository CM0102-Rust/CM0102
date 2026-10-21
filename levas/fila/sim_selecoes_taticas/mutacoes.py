# Mutacoes das convocacoes repetidas, da vaga do escalado e do comentarista. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('c_propria', "            for j in i + 1..50 {", "            for j in i..50 {"),
    ('v_vaga', "        Some(i) => i as u8,", "        Some(i) => i as u8 + 1,"),
    ('m_dobro', "            peso = peso.wrapping_add(peso);", "            peso = peso.wrapping_add(1);"),
]
