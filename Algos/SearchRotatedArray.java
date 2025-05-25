public class SearchRotatedArray {
    /**
     * in a rotated sorted array, return index if found, else -1
     */
    static int searchInRotatedSortedArray(int[] arr, int target) {
        for (int i = 0; i < arr.length; i++) {
            if (arr[i] == target) {
                return i;
            }
        }

        return -1;
    }
}
