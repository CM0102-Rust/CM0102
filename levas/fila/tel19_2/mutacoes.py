# Mutations (tel19_2). The chain runs none (screens batch); for a session of Lucas's with JOGAR_COMPARANDO.bat, on the
# player search screen with more than one page of results (the view, sort, page and pick commands run side by side):
# - the next page button moving two pages: another page in the list's view fields;
# - a column header clicked again keeping the direction: another sort direction.
MUTACOES = [
    ('tel19_2_next_page', '            let page = read_i16(view + 0xe) as i32 + if id == 0x19 { 1 } else { 0xa };',
     '            let page = read_i16(view + 0xe) as i32 + if id == 0x19 { 2 } else { 0xa }; // tel19_2 mutation', ''),
    ('tel19_2_sort_flip', '                if read_u32(f + 4) & v != 0 && read_u32(f + 0xa) == 0 {',
     '                if read_u32(f + 4) & v != 0 && read_u32(f + 0xa) == 1 { // tel19_2 mutation', ''),
]
