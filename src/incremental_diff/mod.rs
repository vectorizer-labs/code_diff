use tree_sitter::Parser;
use super::lang::Parsable;

mod delta;

pub struct IncrementalDiff<'a>
{
    parser : Parser,
    last_source_code : &'a str,
    last_tree : Option<tree_sitter::Tree>
}

impl<'a> IncrementalDiff<'a>
{
    pub fn from_string(source_code : &'a str, language : Parsable) -> IncrementalDiff<'a>
    {
        let mut new_diff = IncrementalDiff::new(language);

        let cursor_index = source_code.len();

        new_diff.update(source_code, cursor_index);

        new_diff
    }

    pub fn new(language : Parsable) -> IncrementalDiff<'a>
    {
        let mut new_parser = Parser::new();

        new_parser.set_language(language.getParserLanguage()).unwrap();

        IncrementalDiff
        {
            parser: new_parser,
            last_source_code : "",
            last_tree : None
        }
    }

    pub fn update(&mut self, new_source_code : &'a str, cursor_index : usize)
    {
        let edit = delta::get_input_edit_from_diff(self.last_source_code, new_source_code, cursor_index);
        
        let new_tree = match self.last_tree.take()
        {
            Some(mut tree) => 
            { 
                tree.edit(&edit); 
                self.parser.parse(new_source_code, Some(&tree))
            },
            None => self.parser.parse(new_source_code, None)
        };

        self.last_tree = new_tree;

        self.last_source_code = new_source_code;
    }

    pub fn print_recursive(&self)
    {
        match &self.last_tree
        {
            Some(tree) => { IncrementalDiff::recursive_descent(&tree.root_node(), String::new()); },
            None => println!("None")
        }
        
    }

    fn recursive_descent(node : &tree_sitter::Node, mut depthString : String)
    {
        //pretty print the node with the tabs in front
        println!("{}{:#?}",depthString, node);

        //add another space for the depth
        depthString.push(' ');

        for n in node.children()
        {
            IncrementalDiff::recursive_descent(&n, depthString.clone());
        }
    }
}