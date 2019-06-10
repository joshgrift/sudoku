#define LINE_MAX 16
#include "sudoku.h"
#include <stdio.h>

/* given a file, return a sudoku structure from the data */
Sudoku importSudoku(FILE *f) {
  Sudoku puzzle;
  char s[sudoku_rows * 2];
  char row = 0;

  if (f == NULL)
    printf("\nCannot open file .\n");
  else
    while (fgets(s, LINE_MAX, f) != NULL) {
      for (int i = 0; i < sudoku_rows; i++)
        puzzle.grid[row][i].value = s[i] - '0';
      row++;
    }

  return puzzle;
}

/* given sudoku structure, print the contents to the screen */
void printSudoku(Sudoku *puzzle) {
  printf("\n\n\n");
  for (int i = 0; i < sudoku_rows; i++) {
    if (i % 3 == 0 && i != 0)
      printf("------|-------|------\n");

    for (int j = 0; j < sudoku_rows; j++) {
      if (j % 3 == 0 && j != 0)
        printf("| ");
      if (puzzle->grid[i][j].value == 0)
        printf("  ");
      else
        printf("%d ", puzzle->grid[i][j].value);
    }
    printf("\n");
  }

  if (puzzle->solved) {
    printf("\n Status: Solved in %d iterations \n", puzzle->iterations);
  } else {
    printf("\nStatus: Failed to solve in %d iterations \n\n",
           puzzle->iterations);
  }
}
