#[cfg(test)]#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_matrix() {
        // Test case 1: 3x2 Matrix with no zeros
        let mut matrix1 = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        zero_matrix(&mut matrix1);
        assert_eq!(matrix1, vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ]);

        // Test case 2: 3x2 Matrix with a single zero
        let mut matrix2 = vec![
            vec![1, 2],
            vec![0, 4],
            vec![5, 6],
        ];
        zero_matrix(&mut matrix2);
        assert_eq!(matrix2, vec![
            vec![0, 2],
            vec![0, 0],
            vec![0, 6],
        ]);

        // Test case 3: 2x3 Matrix with multiple zeros
        let mut matrix3 = vec![
            vec![1, 0, 3],
            vec![4, 5, 0],
        ];
        zero_matrix(&mut matrix3);
        assert_eq!(matrix3, vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);
        let mut matrix4 = vec![
            vec![1, 2, 3],
            vec![4, 0, 6],
            vec![7, 8, 9]
        ];
        zero_matrix(&mut matrix4);
        assert_eq!(matrix4, vec![
            vec![1, 0, 3],
            vec![0, 0, 0],
            vec![7, 0, 9]
        ]);
    }
}

fn zero_matrix(matrix: &mut Vec<Vec<i32>>){
    let n = matrix.len();
    let m = matrix[0].len();
    let mut zero_rows= vec![false; n];
    let mut zero_cols = vec![false; m];

    for i in 0..n{
        for j in 0..m{
            if matrix[i][j] == 0{
                zero_rows[i] = true;
                zero_cols[j] = true;
            }
        }
    }

    for i in 0..n{
        for j in 0..m{
            if zero_rows[i] || zero_cols[j]{
                matrix[i][j] = 0;
            }
        }
    }
}

fn main(){
    let mut matrix3 = vec![
        vec![1, 2, 3],
        vec![4, 0, 6],
        vec![7, 8, 0],
    ];
    zero_matrix(&mut matrix3);
    println!("{:?}", matrix3);
}