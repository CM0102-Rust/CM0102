# Mutacoes (sx01d_4) pela prova do save (mutacoes.py --prova-save N), sem o 4o campo:
# - os registros do registro de historias gravados com 22 bytes em vez de 23: o media.dat sai menor (um byte por
#   registro) e deslocado (o zwrite devolve a quantidade, sem caixa);
# - a contagem dos interesses gravada a partir da capacidade: o media.dat muda quando capacidade e contagem diferem
#   (a tabela cresce de 20 em 20).
MUTACOES = [
    ('sx01d_gravacao_registro_curto',
     'exe::zwrite(read_usize(this + media_fields::STORY_LOG), 0x17, read_u32(log_count), z)',
     'exe::zwrite(read_usize(this + media_fields::STORY_LOG), 0x16 /* mutation */, read_u32(log_count), z)'),
    ('sx01d_gravacao_capacidade',
     'let interest_count = this + media_fields::INTEREST_COUNT;',
     'let interest_count = this + media_fields::INTEREST_CAPACITY; // mutation'),
]
