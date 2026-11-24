# Mutacoes (cx08_19) pela prova do save com um save que tenha a Belgica (CARGA):
# - a carga le so os dois primeiros periodos (o arquivo fica desalinhado; o comparador da carga acusa).
MUTACOES = [
    ('cx08_19_carga_sem_terceiro_periodo',
     'pub(crate) const BEL_SECOND_EXTRA_DATA: [(usize, u32, u32); 4] = [(BEL_SECOND_PERIOD_WON, 1, 1), (BEL_SECOND_PERIOD_WON + 1, 1, 1), (BEL_SECOND_PERIOD_WON + 2, 1, 1), (0xee, 4, 2)];',
     'pub(crate) const BEL_SECOND_EXTRA_DATA: [(usize, u32, u32); 4] = [(BEL_SECOND_PERIOD_WON, 1, 1), (BEL_SECOND_PERIOD_WON + 1, 1, 1), (BEL_SECOND_PERIOD_WON + 2, 1, 0), (0xee, 4, 2)]; // mutation',
     'cx08_19 belgian second division data loaded'),
]
