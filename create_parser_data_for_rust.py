from parser import write_terminals_list, write_table_dict_for_rust
write_terminals_list()
write_table_dict_for_rust('src/grammars/arith_table.html')
write_table_dict_for_rust('src/grammars/full_table.html')
