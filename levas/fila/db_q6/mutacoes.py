# Mutacoes da lista a pedido, da nota relativa e do time reserva. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq6_pedido', "*((v + 0x4e) as *const u8) & 8 != 0", "*((v + 0x4e) as *const u8) & 9 != 0"),
    ('dq6_nota', "(habilidade + 2 * (valor as u8 as i8 as i32))", "(habilidade + 3 * (valor as u8 as i8 as i32))"),
    ('dq6_reservas', "    1 // pode", "    0 // pode"),
]
