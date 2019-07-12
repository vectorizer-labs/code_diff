use tree_sitter::Parser;
use super::lang::Parsable;

pub struct IncrementalDiff
{
    parser : Parser
}

impl IncrementalDiff
{
    pub fn new() -> IncrementalDiff
    {
        return IncrementalDiff
        {
            parser: Parser::new()
        }
    }

    pub fn setLanguage(&mut self, language : Parsable)
    {
        self.parser.set_language(language.getParserLanguage()).unwrap();
    }

    pub fn parse(&mut self, source_code : &str)
    {
        let tree = self.parser.parse(source_code, None).unwrap();

        let root_node = tree.root_node();

        IncrementalDiff::printRecursive(&root_node, 0);
    }

    fn printRecursive(node : &tree_sitter::Node, depth : usize)
    {
        //pretty print the node
        println!("{:#?}", node);

        for n in node.children()
        {
            IncrementalDiff::printRecursive(&n, depth + 1);
        } 
    }
}