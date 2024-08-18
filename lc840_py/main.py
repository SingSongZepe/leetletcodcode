

from typing import List

class Solution:
    def numMagicSquaresInside(self, grid: List[List[int]]) -> int:
        def generate_magic_squares():  
            return [  
                [[8, 1, 6],  
                [3, 5, 7],  
                [4, 9, 2]],  

                [[6, 1, 8],  
                [7, 5, 3],  
                [2, 9, 4]],  
                
                [[4, 9, 2],  
                [3, 5, 7],  
                [8, 1, 6]],  

                [[2, 9, 4],  
                [7, 5, 3],  
                [6, 1, 8]],  

                [[8, 3, 4],  
                [1, 5, 9],  
                [6, 7, 2]],  

                [[4, 3, 8],  
                [9, 5, 1],  
                [2, 7, 6]],  

                [[6, 7, 2],  
                [1, 5, 9],  
                [8, 3, 4]],  

                [[2, 7, 6],  
                [9, 5, 1],  
                [4, 3, 8]]  
            ]  
        def is_magic_square(subgrid):  
            magic_squares = generate_magic_squares()  
            return subgrid in magic_squares  

        def count_magic_squares(grid):  
            rows = len(grid)  
            cols = len(grid[0]) if rows > 0 else 0  
            count = 0  
            
            for i in range(rows - 2):  
                for j in range(cols - 2):  
                    subgrid = [row[j:j+3] for row in grid[i:i+3]]  
                    if is_magic_square(subgrid):  
                        count += 1  
                        
            return count  

        return count_magic_squares(grid)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()