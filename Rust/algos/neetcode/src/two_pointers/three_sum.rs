// 3sum

// for a given array of integers, return a list of lists where arrays[i] + arrays[j] + arrays[k] = 0. 
// the i, j & k are distinct elements. Return the unique result in the response.

// eg-1: Input: nums = [-1,0,1,2,-1,-4]               Output: [[-1,-1,2],[-1,0,1]]

// soln:
// 1. have three valiables for string the current pointers and walk through each by element
// 2. check the condition of arr[i] + arr[j] + arr[k] = 0 & i != j != k, also make sure
// 3. if this case is satisfied, add this in the inner array and push the element to the main array & push the indices i, j, k, into another array called 'indices'
// 4. when you run out of the loops, we call it a day and return the result array

#[allow(dead_code)]
fn three_sum(arr: Vec<i32>) -> Vec<[i32; 3]> {
    let mut result: Vec<[i32; 3]>= Vec::new();
    let mut indices: Vec<i32> = Vec::new();
    // let mut i = 0;
    let mut j: usize = 1;
    let mut k: usize = 2;
    let len = arr.len().try_into().unwrap();

    for i in 0..len {
        while j < len {
            while k < len {
                if arr[i] + arr[j] + arr[k] == 0 && (!indices.contains(&(i as i32)) && !indices.contains(&(j as i32)) && !indices.contains(&(k as i32))) {
                    // its the right pair
                    result.push([arr[i], arr[j], arr[k]]);
                    indices.push(i as i32);
                    indices.push(j as i32);
                    indices.push(k as i32);
                }
                // inc k
                k = k + 1;
            }
            // inc j, reset k
            j = j + 1;
            k = j + 1;
        }
        // inc i, reset j, reset k
        j = i + 2;
        k = j + 1;
    }

    result
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        let sample_1 = [-1,0,1,2,-1,-4];
        let result = three_sum(sample_1.to_vec());
        assert_eq!(result, [[-1,-1,2],[-1,0,1]]);
    }
}