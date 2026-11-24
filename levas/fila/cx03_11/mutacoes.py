# Mutacoes (cx03_11) pela prova do save (mutacoes.py --prova-save N; CARGA), com um save que tenha as ligas da Escocia e
# de Gales (ver o aviso no rotinas.py). Nenhuma abre caixa (as leituras seguem alinhadas).
# - a vaga livre (-1) vira o clube 0 nas copas galesas: a copa carregada muda (o comparador acusa a lista);
# - a Copa da Liga escocesa le os isentos e os zera: a copa carregada muda (o comparador acusa +0xb2).
MUTACOES = [
    ('cx03_11_vaga_livre_vira_clube_zero',
     '        write_usize(slot, if index == -1 { 0 } else { club_at(index as isize).addr() });',
     '        write_usize(slot, if index == -1 { club_at(0).addr() } else { club_at(index as isize).addr() }); // mutation'),
    ('cx03_11_isentos_zerados',
     '    covered!("cx03_11 scottish league cup data loaded");\n    1',
     '    covered!("cx03_11 scottish league cup data loaded");\n    write_u8(this + 0xb2, 0); // mutation\n    1'),
]
