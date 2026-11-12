use crate::parser::types::typ::Type;


pub fn overflowed(
    expected_type : &Type,
    int_const     : i128,
) -> bool{
    let int_const = int_const;
    let type_max_val = max_val(expected_type);
    let type_min_val = min_val(expected_type);
    int_const > type_max_val || int_const < type_min_val

}


fn max_val(ty : &Type) -> i128{
    let max_val = match ty{
        Type::i8 => 127,
        Type::i16=> 32767,
        Type::i32=> 2147483647 ,
        Type::i64=> 9223372036854775807,
        Type::i128=>170141183460469231731687303715884105727,
        _ => unreachable!()
    };

    max_val

}

fn min_val(ty : &Type) -> i128{
    let max_val = match ty{
        Type::i8 => i8::MIN as i128,
        Type::i16=> i16::MIN as i128,
        Type::i32=> i32::MIN as i128,
        Type::i64=> i64::MIN as i128,
        Type::i128=>i64::MIN as i128,
        _ => unreachable!()
    };

    max_val

}