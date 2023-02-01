//! 词法结构: 注释
//!
pub mod note {

    //! 模块注释
    //!

    /// 行注释
    pub fn line_note() {
        println!("line note");
    }

    /** 段落注释 */
    pub fn paragraph_note() {
        println!("paragraph note");
    }

    // 普通注释,另外注释可以多级嵌套
}
