main :
	cc main.c lib/sudoku.c lib/sudoku-helper.c -o main --std=c89
clean :
	rm -f main 