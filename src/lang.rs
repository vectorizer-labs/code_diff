use tree_sitter::Language;
//extern "C" { fn tree_sitter_c() -> Language; }
//extern "C" { fn tree_sitter_rust() -> Language; }
extern "C" { fn tree_sitter_javascript() -> Language; }

pub enum Parsable
{
    Javascript,
    //C,
    //Rust
}

impl Parsable
{
    pub fn getParserLanguage(self) -> Language
    {
        return match self
        {
            Parsable::Javascript => unsafe { tree_sitter_javascript() },
            //Parsable::C =>  unsafe { tree_sitter_c() },
            //Parsable::Rust=> unsafe { tree_sitter_rust() }
        }
    }

}