# Mutacoes (cqi_10). O destino de cada clube no Intertoto (0x633c10, 114 chamadas ate 2002-06-01 com as 8 ligas: as
# rodadas de julho e agosto de 2001) e na Copa da UEFA (0x9030e0, 288 chamadas): o +0x1db do clube (a competicao
# europeia da temporada) fica no bloco dos clubes que a impressao do banco le todo dia e entra na injury_word_0c pela
# competicao do clube e na lista dos inscritos; como na cpz_4 (cpz4_eliminado_na_qualificacao_continua foi acusada em
# 2001-199), mudar a marca deve ser pego no dia. Os contadores ficam colados no efeito, depois das condicoes.
MUTACOES = [
    ('cqi10_vencedor_do_intertoto_fica_fora_da_uefa',
     'core::ptr::write_unaligned((clube + 0x1db) as *mut u32, cqi10_d(0x9cf6f0) as u32); // cqi10: o vencedor passa à UEFA',
     'let _ = clube; // mutacao: o vencedor do Intertoto nao recebe a marca da UEFA',
     'cqi_10 vencedor do intertoto'),
    ('cqi10_eliminado_no_intertoto_continua',
     'leave_euro_competition(clube); // cqi10: quem cai no Intertoto sai da competição europeia',
     'let _ = clube; // mutacao: o eliminado do Intertoto continua na competicao europeia',
     'cqi_10 caiu no intertoto'),
    ('cqi10_eliminado_na_uefa_continua',
     'core::ptr::write_unaligned((clube + 0x1db) as *mut i32, -1); // cqi10: quem termina na UEFA sai da competição europeia',
     'if d != 0xff { core::ptr::write_unaligned((clube + 0x1db) as *mut i32, -1); } // mutacao: o eliminado da UEFA continua',
     'cqi_10 caiu na uefa'),
]
