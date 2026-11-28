# Mutacoes (r7_08_3). TELAS: a cadeia nao roda, e o comparador e so a original (com o plano do status no elenco): jogando
# comparando a copia nao roda. Acusadas na emulacao (scratchpad\r7_08\check8.py squad) e, no jogo, so jogando com a Rust.
# - o rodizio a partir de nenhum status da +20 de moral em vez de +25;
# - 20 de moral por posicao perdida em vez de 25;
# - o contratado recente vira (ambicao + 6) x 4 dias em vez de x 5.
MUTACOES = [
    ('r7_08_3_rodizio',
     '            3 => if asked { refuse(-100) } else { add(25) }, // r7_08_3: rotation from no status',
     '            3 => if asked { refuse(-100) } else { add(20) }, // r7_08_3 mutation',
     'r7_08_3 key player unhappy with his role'),
    ('r7_08_3_posicao',
     '    let d = if old & 0xf0 == 0x30 && status == 4 { -50 } else { ((was as i16 - status as i16) * 25).max(-125) as i32 }; // r7_08_3: 25 per place',
     '    let d = if old & 0xf0 == 0x30 && status == 4 { -50 } else { ((was as i16 - status as i16) * 20).max(-125) as i32 }; // r7_08_3 mutation',
     'r7_08_3 key player unhappy with his role'),
    ('r7_08_3_recente',
     '    if days < (p.ambition() as i32 + 6) * 5 {',
     '    if days < (p.ambition() as i32 + 6) * 4 { // r7_08_3 mutation',
     'r7_08_3 key player unhappy with his role'),
]
