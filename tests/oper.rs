extern crate ndarray;
extern crate num as libnum;

use ndarray::RcArray;
use ndarray::{arr0, rcarr1, rcarr2};

use std::fmt;
use libnum::Float;

fn test_oper(op: &str, a: &[f32], b: &[f32], c: &[f32])
{
    let aa = rcarr1(a);
    let bb = rcarr1(b);
    let cc = rcarr1(c);
    test_oper_arr(op, aa.clone(), bb.clone(), cc.clone());
    let dim = (2, 2);
    let aa = aa.reshape(dim);
    let bb = bb.reshape(dim);
    let cc = cc.reshape(dim);
    test_oper_arr(op, aa.clone(), bb.clone(), cc.clone());
    let dim = (1, 2, 1, 2);
    let aa = aa.reshape(dim);
    let bb = bb.reshape(dim);
    let cc = cc.reshape(dim);
    test_oper_arr(op, aa.clone(), bb.clone(), cc.clone());
}

fn test_oper_arr<A: Float + fmt::Debug, D: ndarray::Dimension>
    (op: &str, mut aa: RcArray<A,D>, bb: RcArray<A, D>, cc: RcArray<A, D>)
{
    match op {
        "+" => {
            assert_eq!(&aa + &bb, cc);
            aa.iadd(&bb);
            assert_eq!(aa, cc);
        },
        "-" => {
            assert_eq!(&aa - &bb, cc);
            aa.isub(&bb);
            assert_eq!(aa, cc);
        },
        "*" => {
            assert_eq!(&aa * &bb, cc);
            aa.imul(&bb);
            assert_eq!(aa, cc);
        },
        "/" => {
            assert_eq!(&aa / &bb, cc);
            aa.idiv(&bb);
            assert_eq!(aa, cc);
        },
        "%" => {
            assert_eq!(&aa % &bb, cc);
            aa.irem(&bb);
            assert_eq!(aa, cc);
        },
        "neg" => {
            assert_eq!(-aa.clone(), cc);
            aa.ineg();
            assert_eq!(aa, cc);
        },
        _ => panic!()
    }
}

#[test]
fn operations()
{
    test_oper("+", &[1.0,2.0,3.0,4.0], &[0.0, 1.0, 2.0, 3.0], &[1.0,3.0,5.0,7.0]);
    test_oper("-", &[1.0,2.0,3.0,4.0], &[0.0, 1.0, 2.0, 3.0], &[1.0,1.0,1.0,1.0]);
    test_oper("*", &[1.0,2.0,3.0,4.0], &[0.0, 1.0, 2.0, 3.0], &[0.0,2.0,6.0,12.0]);
    test_oper("/", &[1.0,2.0,3.0,4.0], &[1.0, 1.0, 2.0, 3.0], &[1.0,2.0,3.0/2.0,4.0/3.0]);
    test_oper("%", &[1.0,2.0,3.0,4.0], &[1.0, 1.0, 2.0, 3.0], &[0.0,0.0,1.0,1.0]);
    test_oper("neg", &[1.0,2.0,3.0,4.0], &[1.0, 1.0, 2.0, 3.0], &[-1.0,-2.0,-3.0,-4.0]);
}

#[test]
fn scalar_operations()
{
    let a = arr0::<f32>(1.);
    let b = rcarr1::<f32>(&[1., 1.]);
    let c = rcarr2(&[[1., 1.], [1., 1.]]);

    {
        let mut x = a.clone();
        let mut y = arr0(0.);
        x.iadd_scalar(&1.);
        y.assign_scalar(&2.);
        assert_eq!(x, a + arr0(1.));
        assert_eq!(x, y);
    }

    {
        let mut x = b.clone();
        let mut y = rcarr1(&[0., 0.]);
        x.iadd_scalar(&1.);
        y.assign_scalar(&2.);
        assert_eq!(x, b + arr0(1.));
        assert_eq!(x, y);
    }

    {
        let mut x = c.clone();
        let mut y = RcArray::zeros((2, 2));
        x.iadd_scalar(&1.);
        y.assign_scalar(&2.);
        assert_eq!(x, c + arr0(1.));
        assert_eq!(x, y);
    }
}
