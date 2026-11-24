# Gravar e liberar o deposito de taticas e o registro antigo de vaga (tactics.cpp, shortlist_manager.cpp; leitor sx20,
# rodada 5, 24/11/2026): Tactics::write_to_save (0x8995d0), Tactics::free_data (0x8961e0) e
# ShortlistOldSlot::ShortlistOldSlot (0x83b360). Rotinas inteiras (0x8995d0..0x899a46 ret 4; 0x8961e0..0x896289 ret;
# 0x83b360..0x83b36e ret).
# Quem chama: write_to_save, o save_game_data (0x82ed7d; o retorno nao e lido); free_data, o free_game_data (0x830054,
# em Rust pelo endereco, que o .qst da prova do save roda depois de gravar); ShortlistOldSlot, o eh vector constructor
# iterator do Shortlist_Manager::load (0x83af8b, pelo endereco), so com save de versao antiga.
# Convencoes conferidas na desmontagem: thiscall; 0x8995d0(this, save) ret 4 -> 0/1, 0x9326e0 zopen cdecl(save,
# "tactics.dat", 1, 0x16, 4), 0x932e10 zwrite, a caixa (tactics.cpp 0x96c sem save, 0x974 sem item, 0x97b..0x9bb nas
# gravacoes) e, depois de aberto, 0x932b00 zclose antes do 0; 0x8961e0(this) ret: 0x9462bb eh vector destructor
# iterator stdcall(blocos, 0x18e3, [blocos-4], 0x5a53c0), 0x944c94 delete(blocos-4), 0x9452ca free das linhas, delete
# das estatisticas; 0x83b360(this) ret -> this, 0x54a110 Date::Date thiscall(this+0x1a).
SAVE = True
ROTINAS = [('tactics_write_to_save', 0x8995d0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('tactics_free_data', 0x8961e0, [0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb]),
           ('shortlist_old_slot_shortlist_old_slot', 0x83b360, [0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x1a])]
DOCS = ('gravar e liberar o deposito de taticas e o registro antigo de vaga',
        'writing and freeing the tactics store and the old slot record',
        [('tactics_write_to_save', 'Tactics::write_to_save', '0x8995d0', 'tactics.cpp', 'thiscall, ret 4',
          'grava as taticas no save: sem save, a caixa (0x96c) e 0; o item tactics.dat (falta: 0x974 e 0); o numero e os blocos das taticas proprias (0x18e3 bytes), dos modelos, das linhas dos clubes (0x91), das estatisticas (contagem em +4, registros de 0x28 de [+0]) e os dez valores padrao de +8; fecha e devolve 1; gravacao curta: a caixa, fecha e 0',
          'writes the tactics into the save: without a save, the box (0x96c) and 0; the tactics.dat item (missing: 0x974 and 0); the count and blocks of the custom tactics (0x18e3 bytes), of the presets, of the club rows (0x91), of the statistics (count at +4, 0x28-byte records from [+0]) and the ten default values from +8; closes and returns 1; a short write: the box, closes and 0'),
         ('tactics_free_data', 'Tactics::free_data', '0x8961e0', 'tactics.cpp', 'thiscall',
          'libera as taticas: os blocos proprios e os modelos (destrutor vetorial e delete), as linhas dos clubes (free) e as estatisticas (delete), zerando cada ponteiro e contagem',
          'frees the tactics: the custom and preset blocks (vector destructor and delete), the club rows (free) and the statistics (delete), clearing each pointer and count'),
         ('shortlist_old_slot_shortlist_old_slot', 'ShortlistOldSlot::ShortlistOldSlot', '0x83b360', 'shortlist_manager.cpp', 'thiscall',
          'monta um registro de vaga do formato antigo (0x24 bytes, de saves antigos): a data padrao em +0x1a; devolve o registro',
          'builds an old-format slot record (0x24 bytes, from old saves): the default date at +0x1a; returns the record')])
