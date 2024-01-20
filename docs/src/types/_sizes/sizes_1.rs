use std::mem::size_of_val;
use std::f32::consts::{E, PI, TAU};
use std::f64::consts::{E as E64, PI as PI64, TAU as TAU64};
use std::any::Any;


#[derive(Debug)]
enum Type {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    ARRAYi8([i8; 1]),
    ARRAYi128([i128; 2])
}

impl Type {
    fn as_any(&self) -> &dyn Any {
        match self {
            Type::Bool(val) => val,
            Type::I8(val) => val,
            Type::I16(val) => val,
            Type::I32(val) => val,
            Type::I64(val) => val,
            Type::I128(val) => val,
            Type::U8(val) => val,
            Type::U16(val) => val,
            Type::U32(val) => val,
            Type::U64(val) => val,
            Type::U128(val) => val,
            Type::F32(val) => val,
            Type::F64(val) => val,
            Type::ARRAYi8(val) => val,
            Type::ARRAYi128(val) => val,
        }
    }
}

fn main() {
    let types: Vec<Type> = vec![
        Type::Bool(true),
        Type::Bool(false),
        Type::I8(-128),
        Type::I8(127),
        Type::I16(-32768),
        Type::I16(32767),
        Type::I32(-2147483648),
        Type::I32(2147483647),
        Type::I64(-9223372036854775808),
        Type::I64(9223372036854775807),
        Type::I128(-170141183460469231731687303715884105728),
        Type::I128(170141183460469231731687303715884105727),
        Type::U8(0),
        Type::U8(255),
        Type::U16(65535),
        Type::U32(4294967295),
        Type::U64(18446744073709551615),
        Type::U128(340282366920938463463374607431768211455),
        Type::F32(E),
        Type::F32(PI),
        Type::F32(TAU),
        Type::F32(f32::NEG_INFINITY),
        Type::F32(f32::INFINITY),
        Type::F64(E64),
        Type::F64(PI64),
        Type::F64(TAU64),
        Type::F64(f64::NEG_INFINITY),
        Type::F64(f64::INFINITY),
        Type::ARRAYi8([1]),
        Type::ARRAYi128([1, 2]),
    ];

    for t in types.iter() {
        let type_size = size_of_val(t.as_any());
        println!("{:?} = {} bytes;", t, type_size);
    }
}
