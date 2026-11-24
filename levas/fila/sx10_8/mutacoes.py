# Mutacoes (sx10_8): telas de taticas, do clube e de treino; a carreira automatica nao as roda, entao a cadeia nao muta
# nesta leva. Provam os comparadores numa sessao do Lucas com o JOGAR_COMPARANDO.bat:
# - check_duplicate_coordinate: uma vaga posterior no mesmo ponto nao marca (Taticas, editar formacao, por duas marcas
#   no mesmo ponto);
# - position_name: o lado fica vazio (Taticas, o nome longo da posicao de uma vaga);
# - position_abbreviation: "so a linha" vale tambem para as de uma letra (telas do clube: DL vira D);
# - flag_name: nenhum nome (Treino, as linhas/lados dos treinos por posicao);
# - check_file_password: a senha do tecnico nunca confere (Taticas, carregar tatica, uma tatica com senha).
# save_to_disk roda so a original no comparador: a mutacao do arquivo so se ve no modo Rust (JOGAR.bat), gravando e
# carregando a tatica (a senha sai sem inverter e volta trocada).
MUTACOES = [
    ('sx10_8_duplicada_posterior',
     'if k > s as i32 { // sx10_8: a later slot on the same point',
     'if k > s as i32 + 100 { // mutation: never later'),
    ('sx10_8_lado_vazio',
     'Some(&(_, name)) => formation_name_text(side.as_mut_ptr() as usize, name), // sx10_8: the side\'s name',
     'Some(&(_, _name)) => {} // mutation: no side'),
    ('sx10_8_so_a_linha',
     'if line_only != 0 && c_string_len(line) > 1 { // sx10_8: a two-letter line stands alone',
     'if line_only != 0 && c_string_len(line) > 0 { // mutation: one letter too'),
    ('sx10_8_bit_sem_nome',
     'Some(name) => formation_name_text(out, name), // sx10_8: the flag\'s name',
     'Some(_name) => {} // mutation: no name'),
    ('sx10_8_senha_nunca_confere',
     'if exe_strcmp(mine, password.as_ptr() as usize) == 0 { // sx10_8: the manager\'s own password',
     'if exe_strcmp(mine, password.as_ptr() as usize) == 99 { // mutation: never equal'),
    ('sx10_8_senha_sem_inverter',
     'formation_flip_password(password); // sx10_8: the password inverted for the file',
     '// mutation: the password written as it is'),
]
