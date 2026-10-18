use crate::par::ast::node::Key;
use crate::par::parser::parser::Parser;


#[allow(warnings)]
#[derive(Debug, Clone, PartialEq)]
pub enum Type{
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
    bool,
    String,
    void,
}
impl Parser{
    pub fn matches_and_insert_var_type(&mut self, key : Key, declared_ty : Type){
        let ty= self.matches_var_type(&declared_ty);
        self.table.insert_var_type(key, ty);
    }

    pub fn is_matches_type(&self, expected_type: Type) -> bool{
        match (expected_type, self.get_expr_type()){
            (Type::i8, Type::i8) => true,
            (Type::i8, Type::i16) => {
                self.out_of_type_range(&Type::i8);
                false
            },
            (Type::i8, Type::i32) => {
                self.out_of_type_range(&Type::i8);
                false
            },
            (Type::i8, Type::i64) => {
                self.out_of_type_range(&Type::i8);
                false
            },
            (Type::i8, Type::i128) => {
                self.out_of_type_range(&Type::i8);
                false
            },
            (Type::i16, Type::i8) => true,
            (Type::i16, Type::i16) => true,
            (Type::i16, Type::i32) => {
                self.out_of_type_range(&Type::i16);
                false
            },
            (Type::i16, Type::i64) => {
                self.out_of_type_range(&Type::i16);
                false
            },
            (Type::i16, Type::i128) => {
                self.out_of_type_range(&Type::i16);
                false
            },

            (Type::i32, Type::i8) => true,
            (Type::i32, Type::i16) => true,
            (Type::i32, Type::i32) => true,
            (Type::i32, Type::i64) => {
                self.out_of_type_range(&Type::i32);
                false
            },
            (Type::i32, Type::i128) => {
                self.out_of_type_range(&Type::i32);
                false
            },

            (Type::i64, Type::i8) => true,
            (Type::i64, Type::i16) => true,
            (Type::i64, Type::i32) => true,
            (Type::i64, Type::i64) => true,
            (Type::i64, Type::i128) => {
                self.out_of_type_range(&Type::i32);
                false
            },

            (Type::i128, Type::i8) => true,
            (Type::i128, Type::i16) => true,
            (Type::i128, Type::i32) => true,
            (Type::i128, Type::i64) => true,
            (Type::i128, Type::i128) => true,

            (Type::f32, Type::f32) => true,
            (Type::f32, Type::f64) => {
                self.out_of_type_range(&Type::i32);
                false
            },

            (Type::f64, Type::f32) => true,
            (Type::f64, Type::f64) => true,

            (Type::bool, Type::bool) => true,
            (Type::String, Type::String) => true,

            (ty1,ty2) if is_int_type(&ty1) &&
                is_flt_type(&ty2) || is_bool_type(&ty2) || is_string_type(&ty2) => {
                self.expected(&ty1, ty2);
                false
            },

            (ty1,ty2) if is_flt_type(&ty1) &&
                is_int_type(&ty2) || is_bool_type(&ty2) || is_string_type(&ty2) => {
                self.expected(&ty1, ty2);
                false
            },

            (ty1,ty2) if is_bool_type(&ty1) &&
                (is_int_type(&ty2) || is_flt_type(&ty2) || is_string_type(&ty2)) => {
                self.expected(&ty1, ty2);
                false
            },

            (ty1,ty2) if is_string_type(&ty1) &&
                (is_flt_type(&ty2) || is_flt_type(&ty2) || is_bool_type(&ty2)) => {
                self.expected(&ty1, ty2);
                false
            },

            _ => panic!("[Line {:?}] Unknown Type", self.current_line())

        }
    }

    pub fn get_expr_type(&self ) -> Type{
        let expr_type = if self.is_int(){
            self.get_int_type()
        }else if self.is_flt(){
            self.get_flt_type()
        }else if self.is_bool(){
            Type::bool
        }else if self.is_string(){
            Type::String
        }else{
            panic!("[Line {:?}] Mismatch Type", self.current_line())
        };

        expr_type

    }

    pub fn matches_var_type(&self, declared_ty : &Type) -> Type{

        let expr_type = self.get_expr_type();


        match (declared_ty, expr_type){
            (Type::void, Type::i8) => Type::i32,
            (Type::void, Type::i16) => Type::i32,
            (Type::void, Type::i32) => Type::i32,
            (Type::void, Type::i64) => Type::i64,
            (Type::void, Type::i128) => Type::i128,
            (Type::void, Type::f32) => Type::f64,
            (Type::void, Type::f64) => Type::f64,
            (Type::void, Type::bool) => Type::bool,
            (Type::void, Type::String) => Type::String,

            (Type::i8, Type::i8) => Type::i8,
            (Type::i8, Type::i16) => self.out_of_type_range(&Type::i8),
            (Type::i8, Type::i32) => self.out_of_type_range(&Type::i8),
            (Type::i8, Type::i64) => self.out_of_type_range(&Type::i8),
            (Type::i8, Type::i128) => self.out_of_type_range(&Type::i8),

            (Type::i16, Type::i8) => Type::i16,
            (Type::i16, Type::i16) => Type::i16,
            (Type::i16, Type::i32) => self.out_of_type_range(&Type::i16),
            (Type::i16, Type::i64) => self.out_of_type_range(&Type::i16),
            (Type::i16, Type::i128) => self.out_of_type_range(&Type::i16),

            (Type::i32, Type::i8) => Type::i32,
            (Type::i32, Type::i16) => Type::i32,
            (Type::i32, Type::i32) => Type::i32,
            (Type::i32, Type::i64) => self.out_of_type_range(&Type::i32),
            (Type::i32, Type::i128) => self.out_of_type_range(&Type::i32),

            (Type::i64, Type::i8) => Type::i64,
            (Type::i64, Type::i16) => Type::i64,
            (Type::i64, Type::i32) => Type::i64,
            (Type::i64, Type::i64) => Type::i64,
            (Type::i64, Type::i128) => self.out_of_type_range(&Type::i32),

            (Type::i128, Type::i8) => Type::i128,
            (Type::i128, Type::i16) => Type::i128,
            (Type::i128, Type::i32) => Type::i128,
            (Type::i128, Type::i64) => Type::i128,
            (Type::i128, Type::i128) => Type::i128,

            (Type::f32, Type::f32) => Type::f32,
            (Type::f32, Type::f64) => self.out_of_type_range(&Type::f32),

            (Type::f64, Type::f32) => Type::f32,
            (Type::f64, Type::f64) => Type::f64,

            (Type::bool, Type::bool) => Type::bool,

            (Type::String, Type::String) => Type::String,

            (ty1,ty2) if is_int_type(&ty1) &&
                ((is_flt_type(&ty2) || is_bool_type(&ty2) || is_string_type(&ty2))) => {
                self.expected(ty1, ty2)
            },

            (ty1,ty2) if is_flt_type(&ty1) &&
                ((is_int_type(&ty2) || is_bool_type(&ty2) || is_string_type(&ty2))) => {
                self.expected(ty1, ty2)
            },

            (ty1,ty2) if is_bool_type(&ty1) &&
                ((is_int_type(&ty2) || is_flt_type(&ty2) || is_string_type(&ty2))) => {
                self.expected(ty1, ty2)
            },

            (ty1,ty2) if is_string_type(&ty1) &&
                ((is_flt_type(&ty2) || is_flt_type(&ty2) || is_bool_type(&ty2))) => {
                self.expected(ty1, ty2)
            },

            _ => panic!("[Line {:?}] Unknown Type", self.current_line())

        }

    }


    fn get_int_type(&self) -> Type{
        let value : i128 = self.ast.max_value.parse().unwrap();

        match value{
            -128..=127 => Type::i8,
            -32768..=32767 => Type::i16,
            -2147483648..=2147483647 => Type::i32,
            -9223372036854775808..=9223372036854775807 => Type::i64,
            -170141183460469231731687303715884105728..=170141183460469231731687303715884105727 => Type::i128,
        }
    }

    pub fn update_int_max(&mut self, key: Key) {
        let interval = &self.lexer.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let new_val: i128 = match self.lexer.input[start..end].parse() {
            Ok(val) => val,
            Err(val) => {
                panic!("[Line {:?}] Expected Int but find : {:?}", self.current_line(), val)
            }
        };

        let max_val: i128 = match self.ast.max_value.parse() {
            Ok(val) => val,
            Err(val) => {
                panic!("[Line {:?}] Expected Int but find : {:?}", self.current_line(), val)
            }
        };

        if new_val > max_val {
            self.ast.max_value.clear();
            self.ast.max_value.push_str(&self.lexer.input[start..end]);
        }
    }

    fn get_flt_type(&self) -> Type{
        let value : f64 = self.ast.max_value.parse().unwrap();

        match value{
            -3.40282347e+38..=3.40282347e+38 => Type::f32,
            -1.7976931348623157e+308..=1.7976931348623157e+308=>Type::f64,
            _ => panic!(
                "[Line {:?}] Mismatch Float value {:?}", self.current_line(), value
            )
        }
    }


    pub fn update_flt_max(&mut self, key: Key) {
        let interval = &self.lexer.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let new_val: f64 = match self.lexer.input[start..end].parse() {
            Ok(val) => val,
            Err(val) => {
                panic!("[Line {:?}] Expected Float but find : {:?}", self.current_line(), val)
            }
        };

        let max_val: f64 = match self.ast.max_value.parse() {
            Ok(val) => val,
            Err(val) => {
                panic!("[Line {:?}] Expected Float but find : {:?}", self.current_line(), val)
            }
        };

        if new_val > max_val {
            self.ast.max_value.clear();
            self.ast.max_value.push_str(&self.lexer.input[start..end]);
        }
    }

    fn expected(&self, expected : &Type, found_ty : Type) -> Type{
        panic!("[Line {:?}] Expected {:?}, but found {:?}", self.current_line(), expected, found_ty)
    }

    fn out_of_type_range(&self, declared_type: &Type) -> Type{
        panic!("[Line {:?}] Out of Type Range {:?}", self.current_line(), declared_type)
    }

    pub fn is_int(&self) -> bool{
        self.ast.int_node_sum == self.ast.regular_op_sum + 1 
            && self.ast.int_node_sum & self.ast.flt_node_sum == 0 
            && self.ast.bool_node_sum == 0 
            && self.ast.compare_op_sum == 0
            && !self.ast.is_string
    }
    
    pub fn is_flt(&self) -> bool{
        self.ast.flt_node_sum == self.ast.regular_op_sum + 1
            &&  self.ast.int_node_sum & self.ast.flt_node_sum == 0
            && self.ast.bool_node_sum == 0
            && self.ast.compare_op_sum == 0
            && !self.ast.is_string
    }

    pub fn is_bool(&self) -> bool{
        self.ast.bool_node_sum == self.ast.compare_op_sum + 1
            && self.ast.int_node_sum & self.ast.flt_node_sum == 0
            && !self.ast.is_string
    }


    pub fn is_string(&self) -> bool{
        self.ast.compare_op_sum == 0
            && self.ast.bool_node_sum == 0
            && self.ast.int_node_sum == 0
            && self.ast.flt_node_sum == 0
            && self.ast.regular_op_sum == 0
            && self.ast.is_string
    }

}


fn is_int_type(ty : &Type) -> bool{
    matches!(ty, Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128)
}

fn is_flt_type(ty : &Type) -> bool{
    matches!(ty, Type::f32 | Type::f64)
}

fn is_bool_type(ty : &Type) -> bool{
    matches!(ty, Type::bool)
}

fn is_string_type(ty : &Type) -> bool{
    matches!(ty, Type::String)
}




