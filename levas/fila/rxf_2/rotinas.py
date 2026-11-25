# The award manager (award_manager.cpp; reader rxf, round 6, 25/11/2026): the award objects of each nation and the save.
# Proven in the disassembly (prologue to the last ret; no jump tables):
# - 0x4135b0 AWARD_MANAGER::create_nation_awards(file): thiscall, ret 4, no value (SEH frame only for the operator new;
#   no destructor to reproduce). Called by the constructor (0x41335b, file 0, new game) and by load (0x4140c2, the
#   open "awards.dat" item). calloc 0x945501 cdecl(30, 4); each class's constructor thiscall(file) ret 4 on
#   operator new(0x17); key_nation_continent_has_selected_nation 0x6699d0 cdecl(continent record). The nation gates
#   read [G_NATION_TABLE] + key * 0x122 + 0x11c (league_selected) without checking the key.
# - 0x4141a0 AWARD_MANAGER::save(save): thiscall, ret 4; returns no value (eax = what the last call left; the only
#   caller, the save at 0x82ee6e, does not read it). NationAwards::save 0x76a380 thiscall(file) ret 4 (sx04b_7);
#   YEAR_RATINGS::save_to_disk 0x9314e0 thiscall(file) ret 4 (sx05b_7); zopen(save, "awards.dat", 1, 0x16, 4).
# Same bytes in cm0102-1080. Comparers: only the original (they allocate or write the container).
# Runs: create_nation_awards at every career's creation and at every load; save at every save (the save proof writes the
# .qst of a new career).
# No defect of the original found.
SAVE = True
CARGA = True  # the save proof also loads the FeriasTesteSave with and without the routines (prova_save.py)
ROTINAS = [('award_manager_create_nation_awards', 0x4135b0, [0x6a, 0xff, 0x68, 0x06, 0x3d, 0x95, 0x00]),
           ('award_manager_save', 0x4141a0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o gerente de prêmios: os objetos de prêmio de cada nação e a gravação',
        'the award manager: the award objects of each nation and the save',
        [('award_manager_create_nation_awards', 'AWARD_MANAGER::create_nation_awards', '0x4135b0', 'award_manager.cpp', 'thiscall, ret 4',
          'regra: a tabela tem 30 vagas; a Copa do Mundo e os prêmios internacionais existem sempre, os europeus quando há uma nação europeia escolhida, e cada uma de 27 nações (Argentina, Bélgica, Brasil, Dinamarca, Inglaterra, Finlândia, França, Alemanha, Holanda, Irlanda, Itália, Japão, Noruega, Portugal, Escócia, Espanha, Suécia, EUA, Polônia, Grécia, Croácia, Austrália, Irlanda do Norte, Turquia, Rússia, País de Gales e Coreia do Sul) tem os seus só se for jogada; os internacionais, os da Finlândia, da França e da Irlanda usam a classe antiga quando o banco não tem a competição de pessoal da nova; cada objeto é criado novo ou lido do save',
          "rule: the table has 30 slots; the World Cup and the international awards always exist, the European ones when a European nation is chosen, and each of 27 nations (Argentina, Belgium, Brazil, Denmark, England, Finland, France, Germany, Holland, Ireland, Italy, Japan, Norway, Portugal, Scotland, Spain, Sweden, USA, Poland, Greece, Croatia, Australia, Northern Ireland, Turkey, Russia, Wales and South Korea) has its own only when played; the international, Finnish, French and Irish ones use the old class when the database lacks the new one's staff competition; each object is built new or read from the save"),
         ('award_manager_save', 'AWARD_MANAGER::save', '0x4141a0', 'award_manager.cpp', 'thiscall, ret 4',
          'grava no item "awards.dat" do save cada objeto de prêmio e as duas tabelas de notas do ano (a mundial e a das nações escolhidas); sem save ou sem o item, a caixa de erro',
          "writes to the save's \"awards.dat\" item each award object and both year ratings tables (the world one and the selected nations' one); without a save or the item, the error box")])
