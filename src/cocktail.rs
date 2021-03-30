pub fn run<T: Ord>(input: &mut[T]) {
    //var assignment
    let length = input.len();
    let mut swapped = true;
    let mut start = 0;
    let mut end = length-1;
    while swapped {
        swapped = false;

        //forwards swapping
        for i in start..end {
            if input[i] > input[i+1]{
                input.swap(i, i+1);
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
                if input[i] > input[i+1] {
                    input.swap(i, i+1);
                    swapped = true;
                }
            }
        } else {
            for i in (0..end-1).rev() {
                if input[i] > input[i+1] {
                    input.swap(i, i+1);
                    swapped = true;
                }
            }
        }
        
        start += 1; 
    }
}