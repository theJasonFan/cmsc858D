pub fn clog(x: usize) -> usize {
    let mut v = 0_usize;
    while x - 1 >> v != 0 {
        v += 1;
    }
    v
}

pub fn flog(x: usize) -> usize {
    let mut v = 0_usize;
    while x >> v != 0{
        v += 1;
    }
    v - 1
}

pub fn cdiv(a: usize, b: usize) -> usize {
    if a % b == 0 {
        a / b
    } else {
        (a / b) + 1
    }
} 
pub fn cdiv_2(x: usize) -> usize {
    (x >> 1) + (x & 1_usize)
}

pub fn fdiv_2(x: usize) -> usize {
    x >> 1
}


#[cfg(test)]
mod tests {
    use crate::math::*;

    #[test]
    fn test_clog(){
        assert_eq!(clog(1), 0);
        assert_eq!(clog(2), 1);
        assert_eq!(clog(15), 4);
        assert_eq!(clog(16), 4);
        assert_eq!(clog(8), 3);
    }

    #[test]
    fn test_flog(){
        assert_eq!(flog(1), 0);
        assert_eq!(flog(2), 1);
        assert_eq!(flog(15), 3);
        assert_eq!(flog(16), 4);
        assert_eq!(flog(17), 4)

    }

    #[test]
    fn test_cdiv_2(){
        assert_eq!(cdiv_2(1), 1);
        assert_eq!(cdiv_2(10), 5);
        assert_eq!(cdiv_2(9), 5)
    }
}