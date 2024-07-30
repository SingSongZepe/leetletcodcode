// 
function spiralOrder(matrix) {
    var result = [];
    iter(matrix, 0, result);
    return result;
}
;
function iter(matrix, from, result) {
    var r = matrix.length;
    var c = matrix[0].length;
    // if ((r % 2 == 0 && from >= r / 2) || (c % 2 == 0 && from >= c / 2) || (r % 2 == 1 && from >= (r - 1) / 2) || (c % 2 == 1 && from >= (c - 1) / 2)) {
    //     return;
    // }
    if (from >= r / 2 || from >= c / 2) {
        return;
    }
    // top
    for (var i = from; i < c - from; i++) {
        result.push(matrix[from][i]);
    }
    // right
    for (var i = from + 1; i < r - from; i++) {
        result.push(matrix[i][c - 1 - from]);
    }
    // bottom
    if (from != (r-1) / 2 && from != (c-1) / 2) {
        for (var i = c - 2 - from; i >= from; i--) {
            result.push(matrix[r - 1 - from][i]);
        }
        // left
        for (var i = r - 2 - from; i > from; i--) {
            result.push(matrix[i][from]);
        }
    }
    iter(matrix, from + 1, result);
}
var test1 = function () {
    var matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    var result = spiralOrder(matrix);
    console.log(result);
};
var test2 = function () {
    var matrix = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12]
    ];
    var result = spiralOrder(matrix);
    console.log(result);
};
test1();
test2();
var test_matrix = function () {
    var matrix = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12]
    ];
    console.log(matrix.length);
    console.log(matrix[0].length);
};
// test_matrix();
// test("t1", () => {
//     const matrix = [
//         [1, 2, 3],
//         [4, 5, 6],
//         [7, 8, 9]
//     ];
//     var result = spiralOrder(matrix);
//     console.log(result);
// })
// test("t1", () => {
//     const matrix = [
//         [1,2,3,4],
//         [5,6,7,8],
//         [9,10,11,12]
//     ];
//     var result = spiralOrder(matrix);
//     console.log(result);
// })
