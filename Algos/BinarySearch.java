public class BinarySearch {
    // binary search
    static int binarySearch(int[] arr, int target) {
        System.out.println("A random cout");
        int low = 0, high = arr.length - 1;
        // int mid = (low + high) / 2;
        // System.out.println("len : " + high);

        while (low <= high) {
            int mid = (low + high) / 2; // CAUTION: when low moves right on a very large array,
            // low + high will overflow the range
            if (arr[mid] == target) {
                return mid;
            } else if (arr[mid] < target) { // look right
                low = mid + 1;
            } else if (arr[mid] > target) { // look left
                high = mid - 1;
            } else {
                // System.out.println("Mid : " + arr[mid]);
            }
        }

        return -1;
    }
}
