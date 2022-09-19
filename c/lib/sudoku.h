#include <stdio.h>
#define sudoku_rows 9

typedef struct {
  int value;
  int possible[10];
} Tile;

typedef struct {
  Tile grid[9][9];
  int solved;
  int iterations;
} Sudoku;

/* solve functions */
int squareHasValue(Sudoku *puzzle, int square_row, int square_col, int value);
int rowHasValue(Sudoku *puzzle, int row, int value);
int colHasValue(Sudoku *puzzle, int column, int value);

/* control  functions */
void solve(Sudoku *puzzle, int iterations);
int check(Sudoku *puzzle);

/* helpers */
Sudoku importSudoku(FILE *f);
void printSudoku(Sudoku *puzzle);
