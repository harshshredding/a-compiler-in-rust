from bs4 import BeautifulSoup
import sys
from IPython.core import ultratb

sys.excepthook = ultratb.FormattedTB(color_scheme='Linux', call_pdb=False)


def get_terminals_list():
    # READ L1 TABLE
    with open('table.txt', 'r') as table_file:
        html_content = table_file.read()
    soup = BeautifulSoup(html_content, 'lxml')
    terminals_row = soup.find('tr')
    terminal_list = terminals_row.find_all('terminal')
    terminal_list = [terminal.text for terminal in terminal_list]
    return terminal_list


def get_table_dict():
    # READ L1 TABLE
    with open('table.txt', 'r') as table_file:
        html_content = table_file.read()
    soup = BeautifulSoup(html_content, 'lxml')
    terminals_row = soup.find('tr')
    terminal_list = terminals_row.find_all('terminal')
    terminal_list = [terminal.text for terminal in terminal_list]

    table_dict = {}
    non_terminal_rows = soup.find_all('tr')
    # ignore header
    non_terminal_rows = non_terminal_rows[1:]
    for non_terminal_row in non_terminal_rows:
        non_terminal = non_terminal_row.find('th').text
        productions = non_terminal_row.find_all('td')
        productions = [(terminal_list[i], production.text) for i, production in enumerate(productions) if
                       len(production.text) > 1]
        production_dict = {token: production for token, production in productions}
        assert non_terminal not in table_dict
        table_dict[non_terminal] = production_dict
    assert len(table_dict) == len(non_terminal_rows)
    return table_dict


def print_production(derived_parts, production_parts, focus_idx, head, tail):
    ret = 'START -> '
    ret += ' '.join(head) + ' '
    ret += ' '.join(derived_parts)
    ret += ' *' + production_parts[focus_idx] + '* '
    ret += ' '.join(production_parts[(focus_idx + 1):])
    ret += ' ' + ' '.join(tail)
    print(ret)


def parse_helper(table_dict, tokens, curr_non_terminal, head, tail, terminal_list):
    productions_dict = table_dict[curr_non_terminal]
    production = productions_dict[tokens[0]]
    production_parts = production.split()
    assert len(production_parts) >= 3, f"{production_parts}"
    assert production_parts[0] == curr_non_terminal
    assert production_parts[1] == '→'
    assert production_parts[0].isupper(), f"reached {production}"
    derived_parts = []
    for focus_idx, new_non_terminal in enumerate(production_parts[2:]):
        print_production(
            derived_parts=derived_parts,
            production_parts=production_parts,
            focus_idx=focus_idx + 2,
            head=head,
            tail=tail
        )
        if new_non_terminal.islower():
            if new_non_terminal != '&epsilon':
                assert new_non_terminal == tokens[0]
                tokens.pop(0)
                derived_parts.append(new_non_terminal)
        else:
            assert new_non_terminal.isupper(), f"'{new_non_terminal}' is not a non-terminal"
            derivation = parse_helper(
                table_dict, tokens, new_non_terminal,
                head=(head + derived_parts),
                tail=(production_parts[(3 + focus_idx):] + tail),
                terminal_list=terminal_list
            )
            for terminal in derivation:
                assert terminal in terminal_list
            derived_parts = derived_parts + derivation
    return derived_parts


def parse(table_dict, tokens):
    tokens = tokens + ['eof']
    terminal_list = get_terminals_list()
    print("Starting parse")
    parse_helper(table_dict, tokens, 'START', [''], [''], terminal_list=terminal_list)
    print("Parsed sucessfully!")
    assert not len(tokens), "all tokens should have been consumed"


def get_calgary_token(token) -> str:
    match token:
        case 'Function':
            return 'function'
        case 'Identifier':
            return 'id'
        case 'OpenParenthesis':
            return 'lpar'
        case 'CloseParenthesis':
            return 'rpar'
        case 'Colon':
            return 'colon'
        case 'IntegerKeyword':
            return 'integer'
        case 'OpenSquareBracket':
            return 'lsqbr'
        case 'CloseSquareBracket':
            return 'rsqbr'
        case 'Comma':
            return 'comma'
        case 'LocalVar':
            return 'localvar'
        case 'SemiColon':
            return 'semi'
        case 'Arrow':
            return 'arrow'
        case 'Void':
            return 'void'
        case 'OpenCurly':
            return 'lcurbr'
        case 'CloseCurly':
            return 'rcurbr'
        case 'EqualsSymbol':
            return 'equal'
        case 'IntLit':
            return 'intlit'
        case 'While':
            return 'while'
        case 'LessThan':
            return 'lt'
        case 'GreaterThan':
            return 'gt'
        case 'Minus':
            return 'minus'
        case 'If':
            return 'if'
        case 'Plus':
            return 'plus'
        case 'Then':
            return 'then'
        case 'Else':
            return 'else'
        case 'Write':
            return 'write'
        case 'Read':
            return 'read'
        case 'Class':
            return 'class'
        case 'Public':
            return 'public'
        case 'Private':
            return 'private'
        case 'FloatKeyword':
            return 'float'
        case 'IsA':
            return 'isa'
        case 'Attribute':
            return 'attribute'
        case 'Constructor':
            return 'constructor'
        case 'Sr':
            return 'sr'
        case 'Return':
            return 'return'
        case 'Asterix':
            return 'mult'
        case 'Period':
            return 'dot'
        case 'FloatLit':
            return 'floatlit'
        case 'LessThanOrEq':
            return 'leq'
        case 'GreaterThanOrEq':
            return 'geq'
        case _:
            raise RuntimeError(f"No way to handle tokenizer output '{token}'")


def to_calgary(tokens: list[str], terminal_list) -> list[str]:
    calgary_tokens = [get_calgary_token(token) for token in tokens]
    for token in calgary_tokens:
        assert token in terminal_list
    return calgary_tokens


def read_tokens_file(tokens_file_path) -> list[str]:
    with open(tokens_file_path, 'r') as tokens_file:
        return [token.strip() for token in tokens_file]


def main():
    tokens = read_tokens_file(sys.argv[1])
    terminal_list = get_terminals_list()
    table_dict = get_table_dict()
    tokens = to_calgary(tokens, terminal_list)
    parse(table_dict, tokens)


main()
