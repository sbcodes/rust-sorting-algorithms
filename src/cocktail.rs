pub fn run(input: std::vec::Vec<i32>) -> std::vec::Vec<i32> {
    //var assignment
    let mut output = input;
    let length = output.len();
    let mut swapped = true;
    let mut start = 0;
    let mut end = length-1;
    while swapped {
        swapped = false;

        //forwards swapping
        for i in start..end {
            if output[i] > output[i+1]{
                output.swap(i, i+1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }

        end -= 1;

        //not entirely happy with this solution but it avoids subtracting from zero
        //backward swapping
        if start >= 1 {
            for i in (start-1..end-1).rev() {
                if output[i] > output[i+1] {
                    output.swap(i, i+1);
                    swapped = true;
                }
            }
        } else {
            for i in (0..end-1).rev() {
                if output[i] > output[i+1] {
                    output.swap(i, i+1);
                    swapped = true;
                }
            }
        }
        
        start += 1; 
    }
    output
}