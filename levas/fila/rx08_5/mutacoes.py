# Mutacoes (rx08_5), ENG ate 2005-12-31:
# - as selecoes do grupo na ordem inversa do sorteio: outra ordem das linhas e outro calendario do grupo (a carreira
#   acusa; o comparador so confere a contagem);
# - seis vagas na repescagem com uma isenta (em vez de oito): menos segundos na repescagem (a carreira acusa nos jogos
#   de novembro de 2005);
# - a mini-liga dos segundos so com um turno (n x (n - 1)): acusada pelo comparador da tabela dos segundos (e pela
#   carreira quando muda quem vai a repescagem).
MUTACOES = [
    ('rx08_5_ordem_inversa',
     'write_usize(list + k * 4, read_usize(drawn + k * 4)); // rx08_5: the drawn nations in draw order',
     'write_usize(list + k * 4, read_usize(drawn + (size as usize - 1 - k) * 4)); // rx08_5 mutation',
     'rx08_5 european group stage'),
    ('rx08_5_repescagem_de_seis',
     'let n: i16 = if read_u8(this + WC_EUROPE_EXEMPT) <= 1 { 8 } else { 6 }; // rx08_5: eight play-off places with up to one exempt nation',
     'let n: i16 = if read_u8(this + WC_EUROPE_EXEMPT) < 1 { 8 } else { 6 }; // rx08_5 mutation',
     'rx08_5 european play-off'),
    ('rx08_5_um_turno',
     'n.wrapping_sub(1).wrapping_mul(n).wrapping_mul(2) as u32, read_usize(this + 0x10)) // rx08_5: both legs between the four',
     'n.wrapping_sub(1).wrapping_mul(n) as u32, read_usize(this + 0x10)) // rx08_5 mutation',
     'rx08_5 european runner-up row copied'),
]
