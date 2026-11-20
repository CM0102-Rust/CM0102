# Mutacoes (mwb_24): as duas copas galesas em grupos criadas com o ano seguinte (as rodadas de 2001/02 caem em
# 2002/03: a carreira nao as joga).
MUTACOES = [
    ('mwb24_ano_seguinte', "this.set_year(year as u16 as i16); // mwb24: the cup's year", 'this.set_year((year as u16 as i16).wrapping_add(1)); // mutation: next year', 'mwb_24 welsh league cup new'),
]
