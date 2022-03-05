fn main() {
    let a = br"CATCG";
    let b = br"GTACCGTC";
    println!("{:?}", String::from_utf8(lcs(a, b)));
}

/**
* 字符串的最长公共子序列问题
* 最终的结果中将字符拆分成两段，每一段一定是当前段对应字符串的字段的最长子序列
* 是典型的最优解包含子问题最优解的问题，所以可以采用动态规划的方式来进行结果的计算
*          0    1    2    3    4    5    6    7    8    
*               G    T    A    C    C    G    T    C
* 0        0    0    0    0    0    0    0    0    0    
*    
* 1   C    0    0    0    0    1    1    1    1    1      
*    
* 2   A    0    0    0    1    1    1    1    1    1
*
* 3   T    0    0    1    1    1    1    1    2    2
*    
* 4   C    0    0    1    1    2    2    2    2    2
*
* 5   G    0    1    1    1    2    2    3    3    3
*
*/
fn lcs(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    if a.is_empty() || b.is_empty() {
        return result;
    }
    let ch = a[0];
    if ch == b[0] {
        // 首先判断第一个字符是否相等
        // 如果相等，则它肯定是最大子序列的一部分
        result.push(ch);
        // 递归获取剩余字符的最优解
        result.extend(lcs(&a[1..], &b[1..]));
    } else {
        // 最优解一定在a往右移1位或者b右移1位对应的字符串结果中
        let first = lcs(&a[1..], b);
        let second = lcs(a, &b[1..]);
        // 剩余字符串的最大公共子序列一定也是结果中的最大公共子序列
        if first.len() > second.len() {
            return first;
        }
        return second;
    }
    result
}
