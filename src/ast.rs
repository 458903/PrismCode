// #[derive(Debug)]
// pub struct CompUnit {//CompUnit::= FuncDef;
//     pub func_def: FuncDef,
// }
#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}
#[derive(Debug)]
pub struct FuncDef {//FuncDef::= FuncType IDENT "(" ")" Block;
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}
#[derive(Debug)]
pub enum FuncType {
    Int
}

#[derive(Debug)]
pub struct Block {//Block::= "{" Stmt "}";
    pub stmt: Stmt
}

#[derive(Debug)]
pub struct Stmt {
    //Stmt::= "return" Number ";";
  // pub num: Number
 pub int_const:i32
}
//
// #[derive(Debug)]
// pub struct Number{
//     pub int_const:i32
// }
