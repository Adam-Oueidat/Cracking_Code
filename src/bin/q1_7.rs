#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        let expected = vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ];

        rotate_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }
}

fn rotate_matrix(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for layer in 0..n / 2 {
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let offset = i - first;
            // save top
            let top = matrix[first][i];
            // left -> top
            matrix[first][i] = matrix[last - offset][first];
            // bottom -> left
            matrix[last - offset][first] = matrix[last][last - offset];
            // right -> bottom
            matrix[last][last - offset] = matrix[i][last];
            // top -> right
            matrix[i][last] = top;
        }
    }
}

fn main(){
    let mut matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    rotate_matrix(&mut matrix)
}

// [0][0] -> [0][2]
// [0][1] -> [1][2]
// [0][2] -> [2][2]
// [1][0] -> [0][1]
// [1][1] -> [1][1]
// [1][2] -> [2][1]
// [2][0] -> [0][0]
// [2][1] -> [1][0]
// [2][2] -> [2][0]
 