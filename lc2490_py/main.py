
class Solution:
    def isCircularSentence(self, sentence: str) -> bool:
        if sentence[0] != sentence[-1]:
            return False
        for i in range(len(sentence)):
            if sentence[i] == ' ':
                if sentence[i+1] != sentence[i-1]:
                    return False
        return True



def main():
    print('Hello World')

if __name__ == '__main__':
    main()