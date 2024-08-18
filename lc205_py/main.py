


class Bimap:
    def __init__(self):
        self.forward = {}
        self.backward = {}

    def insert_forward(self, key, value):
        self.forward[key] = value
    
    def insert_backward(self, value, key):
        self.backward[value] = key

    def get_value(self, key):
        if key in self.forward:
            return self.forward[key]
        return None

    def get_key(self, value):
        if value in self.backward:
            return self.backward[value]
    
    def exists_key(self, key):
        return key in self.forward

    def exists_value(self, value):
        return value in self.backward

class Solution:
    def isIsomorphic(self, s: str, t: str) -> bool:
        mp = Bimap()
        for sc, tc in zip(s, t):
            if mp.exists_key(sc):
                if mp.get_value(sc) != tc:
                    return False
            else:
                mp.insert_forward(sc, tc)

            if mp.exists_value(tc):
                if mp.get_key(tc) != sc:
                    return False
            else:
                mp.insert_backward(tc, sc)
            
        return True


class Solution1:
    def isIsomorphic(self, s: str, t: str) -> bool:
        indexS = [0] * 200 # Stores index of characters in string s
        indexT = [0] * 200 # Stores index of characters in string t
        
        length = len(s) # Get the length of both strings
        
        if length != len(t): # If the lengths of the two strings are different, they can't be isomorphic
            return False
        
        for i in range(length): # Iterate through each character of the strings
            if indexS[ord(s[i])] != indexT[ord(t[i])]: # Check if the index of the current character in string s is different from the index of the corresponding character in string t
                return False # If different, strings are not isomorphic
            
            indexS[ord(s[i])] = i + 1 # updating position of current character
            indexT[ord(t[i])] = i + 1
        
        return True # If the loop completes without returning false, strings are isomorphic

class Solution2:
    def isIsomorphic(self, s: str, t: str) -> bool:
        
        if len(s) != len(t):
            return False

        s_to_t = {}
        t_to_s = {}

        for s_ch, t_ch in zip(s, t):
            if s_ch in s_to_t:
                if s_to_t[s_ch] != t_ch:
                    return False
            else:
                s_to_t[s_ch] = t_ch

            
            if t_ch in t_to_s:
                if t_to_s[t_ch] != s_ch:
                    return False
            else:
                t_to_s[t_ch] = s_ch
        return True

def main():
    print('Hello World')

if __name__ == '__main__':
    main()