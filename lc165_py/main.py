
class Solution:
    def compareVersion(self, version1: str, version2: str) -> int:
        def get(version: str) -> (int, int):
            idx = version.find('.')
            if idx == -1:
                return (-1, int(version))
            else:
                return (idx, int(version[:idx]))

        while 1:
            v1, subversion1 = get(version1)
            version1 = version1[v1+1:]
            v2, subversion2 = get(version2)
            version2 = version2[v2+1:]

            if v1 == -1 and v2 == -1:
                return 0 if subversion1 == subversion2 else (1 if subversion1 > subversion2 else -1)
            elif v1 == -1:
                if subversion1 > subversion2:
                    return 1
                elif subversion1 < subversion2:
                    return -1

                while 1:
                    v2, subversion2 = get(version2)
                    version2 = version2[v2+1:]
                    if v2 == -1 and subversion2 == 0:
                        return 0
                    if subversion2 == 0:
                        continue
                    else:
                        return -1

            elif v2 == -1:
                if subversion1 > subversion2:
                    return 1
                elif subversion1 < subversion2:
                    return -1

                while 1:
                    v1, subversion1 = get(version1)
                    version1 = version1[v1+1:]
                    if v1 == -1 and subversion1 == 0:
                        return 0
                    if subversion1 == 0:
                        continue
                    else:
                        return 1

            else:
                if subversion1 == subversion2:
                    continue
                else:
                    return 1 if subversion1 > subversion2 else -1


class Solution1:
    def compareVersion(self, version1: str, version2: str) -> int:
        parts1, parts2 = version1.split("."), version2.split(".")
        i, j = len(parts1) - 1, len(parts2) - 1
        while parts1 != [] and int(parts1[i]) == 0:
            parts1.pop()
            i -= 1
        while parts2 != [] and int(parts2[j]) == 0:
            parts2.pop()
            j -= 1
        for i in range(len(min(parts1, parts2))):
            int1, int2 = int(parts1[i]), int(parts2[i])
            if int1 > int2:
                return 1
            elif int1 < int2:
                return -1
        if len(parts1) > len(parts2):
            return 1
        elif len(parts2) > len(parts1):
            return -1
        return 0



def main():
    print('Hello World')

if __name__ == '__main__':
    main()