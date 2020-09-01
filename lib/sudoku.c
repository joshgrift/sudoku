#include "sudoku.h"

int rowHasValue(Sudoku *puzzle, int row, int value) {
  for (int i = 0; i < sudoku_rows; i++)
    if (puzzle->grid[row][i].value == value)
      return 1;

  return 0;
}

int colHasValue(Sudoku *puzzle, int column, int value) {
  for (int i = 0; i < sudoku_rows; i++)
    if (puzzle->grid[i][column].value == value)
      return 1;
  return 0;
}

int squareHasValue(Sudoku *puzzle, int row, int column, int value) {
  int i_increase = row * 3;
  int j_increase = column * 3;
  for (int i = 0; i < 3; i++)
    for (int j = 0; j < 3; j++)
      if (puzzle->grid[i + i_increase][j + j_increase].value == value)
        return 1;

  return 0;
}

void solve(Sudoku *puzzle, int iterations) {
  while (!puzzle->solved && puzzle->iterations <= iterations) {
    for (int i = 0; i < sudoku_rows; i++)
      for (int j = 0; j < sudoku_rows; j++)
        if (puzzle->grid[i][j].value == 0) {
          /* START LOOP LOGIC */
          int catch = 0;
          int count = 0;

          for (int k = 1; k <= 9; k++) {
            if (!rowHasValue(puzzle, i, k) && !colHasValue(puzzle, j, k) &&
                !squareHasValue(puzzle, i / 3, j / 3, k)) {
              puzzle->grid[i][j].possible[k] = 1;
            } else {
              puzzle->grid[i][j].possible[k] = 0;
            }
          }

          for (int k = 1; k <= 9; k++)
            if (puzzle->grid[i][j].possible[k] == 1) {
              count++;
              catch = k;
            }

          int fail = 0;
          if (count == 1) {
            puzzle->grid[i][j].value = catch;
          } else {
            for (int k = 1; k <= 9; k++)
              if (puzzle->grid[i][j].possible[k] == 1) {
                int quadrow = i - (i % 3);
                int quadcol = j - (j % 3);

                int matrix[3][3];

                for (int qi = 0; qi < 3; qi++)
                  for (int qj = 0; qj < 3; qj++)
                    matrix[qi][qj] = 0;

                for (int qi = 0; qi < 3; qi++) {
                  for (int qj = 0; qj < 3; qj++) {
                    if (rowHasValue(puzzle, qi + quadrow, k)) {
                      matrix[0][qj] = 1;
                      matrix[1][qj] = 1;
                      matrix[2][qj] = 1;
                    }
                    if (colHasValue(puzzle, qj + quadcol, k)) {
                      matrix[qi][0] = 1;
                      matrix[qi][1] = 1;
                      matrix[qi][2] = 1;
                    }

                    if (puzzle->grid[qi + quadrow][qj + quadcol].value != 0) {
                      matrix[qi][qj] = 1;
                    }
                  }
                }

                int catch_i;
                int catch_j;
                int count = 0;

                for (int qi = 0; qi < 3; qi++) {
                  for (int qj = 0; qj < 3; qj++) {
                    if (matrix[qi][qj] == 0) {
                      printf("%d ", matrix[qi][qj]);
                      count++;
                      catch_i = qi;
                      catch_j = qj;
                    }
                  }
                }

                if (count == 1)
                  puzzle->grid[catch_i + quadrow][catch_j + quadcol].value = k;
              }
          }

          /* END LOOP LOGIC */
        }
    puzzle->iterations++;
    puzzle->solved = check(puzzle);
  }
}

int check(Sudoku *puzzle) {
  for (int i = 0; i < sudoku_rows; i++)
    for (int j = 0; j < sudoku_rows; j++)
      if (puzzle->grid[i][j].value == 0)
        return 0;

  return 1;
}
