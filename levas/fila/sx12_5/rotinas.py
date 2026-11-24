# O save das lesoes, dos tecnicos humanos e das competicoes das nacoes-chave (injury.cpp, human_manager.cpp,
# key_nation.cpp; leitor sx12, rodada 5, 23/11/2026): Injury::save 0x62b2d0, HUMAN_MANAGER::save 0x5ff9e0 e
# key_nation_save_all_comps 0x6692f0 (o passo 0x19 do save_game_data 0x82e430, setup.rs SaveStep::Competicoes).
# Convencoes conferidas na desmontagem:
# - Injury::save: thiscall(save), ret 4, o eax de volta nao e lido; 0x9326e0 zopen cdecl(save, "injury.dat", 1, 0x16,
#   4); 0x932e10 zwrite cdecl(ptr, tamanho, quantidade, z); 0x932b00 zclose cdecl(&z); 0x68d1b0 LLIST::get_next
#   thiscall. A contagem da lista vai como palavra (mov dx, word [lista]; so 2 bytes gravados). Linhas das caixas:
#   0x227, 0x22f (sem item), 0x236, 0x23e, 0x246, 0x24e, 0x259, 0x262, 0x26e, 0x27d (com zclose), 0x28f (sem zclose:
#   defeito do original, o item fica aberto).
# - HUMAN_MANAGER::save: thiscall(save), ret 4, eax 1 ou 0; as gravacoes de cada registro nao sao conferidas; as tres
#   ultimas vagas de tatica de cada registro vao no fim (depois de +8 e +0xc), de 0x3d9a.
# - key_nation_save_all_comps: cdecl, eax 1; o metodo +0x1c de cada competicao thiscall(1).
# Comparadores: so a original (gravam no conteiner e nos temporarios); a prova e a do save (.qst com e sem a leva).
SAVE = True
ROTINAS = [('injury_save', 0x62b2d0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('human_manager_save', 0x5ff9e0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('key_nation_save_all_comps', 0x6692f0, [0x56, 0x57, 0xbe, 0x6c, 0x3d, 0xb6, 0x00])]
DOCS = ('o save das lesoes, dos tecnicos humanos e das competicoes das nacoes-chave',
        'saving the injuries, the human managers and the key nation competitions',
        [('injury_save', 'Injury::save', '0x62b2d0', 'injury.cpp', 'thiscall, ret 4',
          'grava no item injury.dat do save os registros fisicos de todas as pessoas (31 bytes), as lesoes (contagem e 0x30 bytes cada), a fila das noticias de volta (contagem e 22 bytes cada), os surtos de doenca (contagem e 17 bytes cada) e a lista das pancadas das partidas do dia (contagem em palavra e 6 bytes cada); cada gravacao conferida: erro, a caixa "Error" com a linha e o item fechado; defeito do original: erro numa pancada deixa o item aberto',
          "writes into the save's injury.dat item every person's physical record (31 bytes), the injuries (count and 0x30 bytes each), the recovery news queue (count and 22 bytes each), the illness outbreaks (count and 17 bytes each) and the list of the day's match knocks (word count and 6 bytes each); each write checked: on error the \"Error\" box with its line and the item closed; defect of the original: an error on a knock leaves the item open"),
         ('human_manager_save', 'HUMAN_MANAGER::save', '0x5ff9e0', 'human_manager.cpp', 'thiscall, ret 4',
          'grava no item human_manager.dat a quantidade de registros e, de cada tecnico humano, as duas primeiras vagas de tatica, as ferias, as opcoes, o rascunho de contrato, as propostas, a senha, o controle pelo computador, a velocidade da narracao, a partida assistida, as pastas e os arquivos recentes, a vista do elenco, as opcoes de ferias, a escalacao guardada e os apelidos; depois dois contadores e, no fim, as tres ultimas vagas de tatica de cada um; devolve 1, ou 0 depois da caixa "Error"',
          'writes into the human_manager.dat item the record count and, for each human manager, the first two tactic slots, the holiday, the options, the draft contract, the offers, the password, the computer control, the commentary speed, the watched fixture, the folders and recent files, the squad view, the holiday options, the saved line-up and the nicknames; then two counters and, at the end, the last three tactic slots of each; returns 1, or 0 after the "Error" box'),
         ('key_nation_save_all_comps', 'key_nation_save_all_comps', '0x6692f0', 'key_nation.cpp', 'cdecl',
          'manda cada competicao de cada linha da tabela das nacoes-chave gravar o seu arquivo temporario; devolve 1',
          'tells every competition of each key nation table row to write its temporary file; returns 1')])
