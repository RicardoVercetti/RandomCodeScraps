class ValidSudoku {
    public boolean isValidSudoku(char[][] board) {
        // check if values between 0-9 don't repeat

        // check per rows
        for(int i=0; i<board.length; i++) {
            List<Character> found = new ArrayList();
            for(char a: board[i]) {
                if(a != '.' && found.contains(a)) {
                    return false;
                } else if(a != '.') {
                    found.add(a);
                }
            }
        }
        
        // check per column, keep row the same until col ends
        // int col = 0;
        // int row = 0;
        for(int col=0; col<board.length; col++) {
            for(int row=0; row<board[0].length; row++) {
                Set<Character> found = new HashSet<>();
                if(board[row][col] != '.' && !found.add(board[row][col])) {
                    return false;
                }
            }
        }

        // check all 3x3
        // r_n = 0
        // c_n = 0
        // rows 0,1,2  (r_n+3)
        // cols 0,1,2  (c_n+3)
        int r_n = 0;
        int c_n = 0;
        int max_r = board.length;
        int max_c = board[0].length;

        while(true) {
            // rows to check - 0,1,2
            // cols to check - 0,1,2
            int[] rows = new int[] {0,1,2};
            int[] cols = new int[] {0,1,2};
            Set<Character> found = new HashSet();
            for(int a: rows) {
                for(int b: cols) {
                    char val = board[a+r_n][b+c_n];
                    if(val != '.' && !found.add(val)) {
                        return false;
                    }
                }
            }

            if(r_n+3 < max_r) {
                r_n +=3;
            } else if(r_n+3 >= max_r && !(c_n+3>=max_c)) {
                r_n = 0;
                c_n += 3;
            } else if(c_n+3 >= max_c) {
                break;
            }
        }
    return true;
    }

}
