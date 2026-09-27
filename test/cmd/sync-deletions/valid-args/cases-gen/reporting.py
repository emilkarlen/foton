class DeletedFile:
    def __init__(self, path_comps_before: list[str], file_name: str):
        self.path_comps_before = path_comps_before
        self.file_name = file_name

def deletion_lines(root_dir: str, deletions: list[DeletedFile]) -> list[str]:
    return [
        'rm \'{}\''.format('/'.join([root_dir] + d.path_comps_before + [d.file_name]))
        for d in deletions
        ]


def move_lines(root_dir: str, move_to_root_dir: str, deletions: list[DeletedFile]) -> list[str]:
    return [
        'mv \'{}\' \'{}\''.format(
            '/'.join([root_dir] + d.path_comps_before + [d.file_name]),
            '/'.join([move_to_root_dir] + d.path_comps_before),
            )
        for d in deletions
        ]
