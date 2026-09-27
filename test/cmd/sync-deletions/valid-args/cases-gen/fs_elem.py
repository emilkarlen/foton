import reporting
import exactly

class FsElem:
    def key_word(self) -> str:
        raise NotADirectoryError()

    def to_str(self, indent: str) -> str:
        raise NotADirectoryError()

    def to_exactly_files_source(self, indent: str, is_assignment: bool = False) -> str:
        raise NotADirectoryError()

    def to_exactly_files_condition(self, indent: str, is_nested: bool) -> str:
        raise NotADirectoryError()

class File(FsElem):
    def key_word(self) -> str:
        return exactly.FC_ELEM_FILE

    def to_str(self, indent: str) -> str:
        return ''

    def to_exactly_files_source(self, indent: str, is_assignment: bool = False) -> str:
        return ''

    def to_exactly_files_condition(self, indent: str, is_nested: bool) -> str:
        return exactly.FM_TYPE_FILE

class Dir(FsElem):
    def __init__(self, name_fs_elem_list: list[tuple[str, FsElem]]):
        self.name_fs_elem_list = tuple(name_fs_elem_list)

    def lookup(self, name: str) -> FsElem:
        for (n, fse) in self.name_fs_elem_list:
            if n == name:
                return fse
        return None

    def key_word(self) -> str:
        return exactly.FC_ELEM_DIR

    def with_deletions_of(self, d: 'Dir') -> 'Dir':
        name_fs_elem_list = []
        for (name, fse) in self.name_fs_elem_list:
            # print("DEBUG: name to maybe del: " + name)
            deleted_fse = d.lookup(name)
            if deleted_fse is None:
                name_fs_elem_list += [(name, fse)]
            else:
                # print("DEBUG: name to del: " + name)
                if isinstance(fse, Dir):
                    if not isinstance(deleted_fse, Dir):
                        exit_failure('deletions: Non matching file elements named \'{}\'\nType is Dir in elements to delete from, but in elements to delete: {}'.format(name, deleted_fse.to_str("")))
                    name_fs_elem_list  += [(name, fse.with_deletions_of(deleted_fse))]

        return Dir(name_fs_elem_list)

    def to_str(self, indent: str) -> str:
        ret_val = "{\n"
        for (ident, val) in self.name_fs_elem_list:
            ret_val += (indent + '  ' + val.key_word() + ' ' + ident + val.to_str(indent + '  ') + '\n')
        return ret_val + '}'

    def to_exactly_files_source(self, indent: str, is_assignment: bool = False) -> str:
        ret_val = indent + "{\n"
        if is_assignment:
            ret_val = ' =\n' + ret_val
        for (ident, val) in self.name_fs_elem_list:
            ret_val += '{}  {} {}{}\n'.format(
                indent,
                val.key_word(),
                exactly.str_const(ident),
                val.to_exactly_files_source(indent + '  ', True),
                )
        return ret_val + indent + '}'

    def to_exactly_files_condition(self, indent: str, is_nested: bool) -> str:
        ret_val = ''
        if is_nested:
            ret_val = '{}\n'.format(exactly.FC_IS_DIR_AND_MATCHES_FULL)
        ret_val += indent + "{\n"
        for (name, val) in self.name_fs_elem_list:
            ret_val += '{}  {} : {}\n'.format(
                indent,
                exactly.str_const(name),
                val.to_exactly_files_condition(indent + '  ', True),
                )
        return ret_val + indent + '}'

    def deletion_elements(self) -> list[reporting.DeletedFile]:
        return self._deletion_elements([])

    def _deletion_elements(self, comps_before: list[str]) -> list[reporting.DeletedFile]:
        ret_val = []
        for (name, fse) in self.name_fs_elem_list:
            if isinstance(fse, File):
                ret_val += [reporting.DeletedFile([] + comps_before, name)]
            elif isinstance(fse, Dir):
                ret_val += fse._deletion_elements(comps_before + [name])
            else:
                exit_failure('Impl error: Unknown type of FsElem: ' + str(fse))
        return ret_val
