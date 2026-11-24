# Uma fase de mata-mata (Cup_Stage) no arquivo temporario da copa (cup_stage.cpp; leitor cx02, rodada 5, 24/11/2026).
# Item do SAVE (CARGA). A fase de uma copa e gravada por Cup::save_cup_to_disk 0x5210f0 (cx01_7) e lida por
# Cup::load_from_disk 0x51fc00 (gra_28): na carreira ENG, as fases do Trofeu das Divisoes Inferiores (secoes e final)
# e as que existirem no FeriasTesteSave.
# - 0x5230e0 Cup_Stage::Cup_Stage thiscall(times, rodadas, &confrontos) ret 0xc -> this (o construtor da carga, chamado
#   por Cup::load_from_disk depois do operator new 0xb2; SEH so para o new): Cup::Cup 0x518640 (mov ecx -> thiscall),
#   a vtable 0x969468, malloc 0x944e46 (cdecl) das tabelas (times x 6, rodadas x 0x68, rodadas x 4 e, por rodada,
#   confrontos x 0x35, as contagens em 16 bits com sinal); as caixas das linhas 0x7e, 0x85, 0x8d e 0x97. Nao grava
#   +0x3c nem +0x3e. Prologo push -1; push imm32 (7 bytes).
# - 0x523380 Cup_Stage::load_from_file thiscall(arquivo) ret 4 -> 1 (2,8 KB, quase tudo as caixas): fread 0x945bfe
#   (cdecl) dos campos, um a um, na ordem da gravacao; a competicao pelo id ([0xae23d0] + id x 0x6b; -1 nulo); as
#   tabelas; os ids dos times viram clubes (club_entries_from_ids do cx01_7); leitura curta: a caixa e o jogo para
#   (game_fatal_cleanup 0x5cc7d0, 0x5a8f60, exit 0x945934(-1): fatal_error_at). Prologo sub esp, 0x208.
# - 0x523eb0 Cup_Stage::save_to_file thiscall(arquivo) ret 4: fwrite 0x945ce6 (cdecl) na mesma ordem; sem tabelas a
#   caixa da linha 0x19a; sem competicao a da linha 0x1b9 e -1. Os clubes do time viram ids e voltam
#   (club_entries_to_ids / club_entries_from_ids do cx01_7). Prologo sub esp, 0x20c.
# Integrar depois do cx01_7. Iguais no cm0102-1080.exe.
# Defeito do original (0x523380): a leitura da tabela de times e conferida contra o numero lido da tabela de rodadas
# (a conferencia da linha 0x163 repete a da 0x15b): uma tabela de times curta passa sem aviso.
# Comparadores: o construtor so a original (aloca), conferindo a vtable e as tabelas; a leitura lado a lado com o
# arquivo de volta a posicao (o objeto, as tabelas, o retorno e a posicao); a gravacao so a original, conferindo que a
# tabela de times volta com os clubes.
SAVE = True
CARGA = True
ROTINAS = [('new_cup_stage_for_load', 0x5230e0, [0x6a, 0xff, 0x68, 0x5b, 0x78, 0x95, 0x00]),
           ('cup_stage_load_from_file', 0x523380, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('cup_stage_save_to_file', 0x523eb0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00])]
DOCS = ('uma fase de mata-mata no arquivo temporário da copa', "a cup stage in the cup's temporary file",
        [('new_cup_stage_for_load', 'Cup_Stage::Cup_Stage', '0x5230e0', 'cup_stage.cpp', 'thiscall, ret 0xc',
          'monta uma fase de mata-mata vazia para a carga: a base da copa e as tabelas de times, de rodadas e dos confrontos de cada rodada, nos tamanhos que vêm do arquivo; falta de memória: a caixa de erro',
          "builds an empty cup stage for loading: the cup base and the team, round and per-round tie tables, in the sizes that come from the file; no memory: the error box"),
         ('cup_stage_load_from_file', 'Cup_Stage::load_from_file', '0x523380', 'cup_stage.cpp', 'thiscall, ret 4',
          'lê a fase do arquivo temporário da copa na ordem da gravação: os campos, a competição, as rodadas, os times (os ids voltam a ser clubes) e os confrontos; leitura curta: a caixa de erro e o jogo para',
          "reads the stage from the cup's temporary file in the order it was written: the fields, the competition, the rounds, the teams (the ids become clubs again) and the ties; a short read: the error box and the game stops"),
         ('cup_stage_save_to_file', 'Cup_Stage::save_to_file', '0x523eb0', 'cup_stage.cpp', 'thiscall, ret 4',
          'grava a fase no arquivo temporário da copa: os campos, o id da competição, as rodadas, os times (como ids) e os confrontos; sem as tabelas, a caixa de erro e nada',
          "writes the stage into the cup's temporary file: the fields, the competition id, the rounds, the teams (as ids) and the ties; without the tables, the error box and nothing")])
CAMPOS = []
INTERESSA = []
