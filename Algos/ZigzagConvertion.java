public class ZigzagConvertion {
    /**
     * String zigzag algorithm
     */
    static String zigZagConvertion(String word, int nrOfRows) {
        boolean isForward = true;
        ArrayList<StringBuffer> allArrays = new ArrayList<StringBuffer>();
        for (int i = 0; i < nrOfRows; i++) {
            allArrays.add(new StringBuffer());
        }

        // do the splitting
        int currentBufferRow = 0;
        for (int currentPos = 0; currentPos < word.length(); currentPos++) {
            // System.out.println("Entry record--" + word.charAt(currentPos));
            allArrays.get(currentBufferRow).append(word.charAt(currentPos));

            // if forward do ++, else do --
            if (isForward && currentBufferRow < (nrOfRows - 1)) {
                currentBufferRow++;
            } else if (isForward && currentBufferRow == (nrOfRows - 1)) {
                currentBufferRow--;
                isForward = !isForward;
            } else if (!isForward && currentBufferRow > 0) {
                currentBufferRow--;
            } else if (!isForward && currentBufferRow == 0) {
                currentBufferRow++;
                isForward = !isForward;
            } else {
                System.out.println("The Bizzare else condition..");
            }
        }

        // now get all the strings one by one
        StringBuffer finalString = new StringBuffer();
        allArrays.stream().forEach(br -> {
            finalString.append(br.toString());
        });

        return finalString.toString();
    }
}
