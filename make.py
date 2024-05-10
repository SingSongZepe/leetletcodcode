import os
import sys
import subprocess

LC_ = 'lc'
LC_TEMPLATE_ = 'lc_template'


def make_template(index) -> bool:
    powershell_command = f"copy lc_template ./lc{index} -recurse"
    result = subprocess.run(["powershell", "-Command", powershell_command], shell=True)
    if result.returncode == 0:
        return True
    else:
        return False

def find_index() -> int:
    index = 1
    while os.path.exists(LC_ + str(index)):
        index += 1
    return index

def exist_index(index) -> bool:
    return os.path.exists(LC_ + str(index))

def main() -> int:
    if not os.path.exists(LC_TEMPLATE_):
        raise Exception('lc_template not found, may be deleted or renamed.')
    if len(sys.argv) > 1: # has a 1
        if not exist_index(sys.argv[1]):
            index = sys.argv[1]
        else:
            raise Exception(f'lc{sys.argv[1]} already exists.')
    else:
        index = find_index()
    if make_template(index):
        print(f'lc{index} created successfully.')
    else:
        print(f'Failed to create lc{index}.')
    return 0


if __name__ == '__main__':
    main()
