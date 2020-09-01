main :
	cc main.c lib/sudoku.c lib/sudoku-helper.c -o main
clean :
	rm -f main 