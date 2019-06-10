#include "sudoku.h"
#include <stdio.h>

int main(int argc, char *args[]) {
  char *filename = "hard.txt\0";
  FILE *fp = fopen(filename, "r");
  Sudoku puzzle = importSudoku(fp);
  fclose(fp);

  solve(&puzzle, 100);

  printSudoku(&puzzle);
}
