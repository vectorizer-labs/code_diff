
pub mod lang;

//#[macro_use]
//extern crate lazy_static;


use tree_sitter::{ Parser, InputEdit };
//use xi_rope::{ engine::Engine, Rope, Interval, DeltaBuilder, RopeDelta };

pub struct Diff
{
    parser : Parser,
    last_source_code : String,
    last_tree : Option<tree_sitter::Tree>
}

impl Diff
{
    pub fn new(language : lang::Parsable) -> Diff
    {
        let mut new_parser = Parser::new();

        new_parser.set_language(language.get_parser_language()).unwrap();

        Diff
        {
            parser: new_parser,
            last_source_code : String::new(),
            last_tree : None
        }
    }

    pub fn update(&mut self, new_source_code : String, edit : InputEdit)
    {
        let new_tree = match self.last_tree.take()
        {
            Some(mut tree) => 
            { 
                tree.edit(&edit); 
                self.parser.parse(new_source_code.clone().as_str(), Some(&tree))
            },
            None => self.parser.parse(new_source_code.clone(), None)
        };

        self.last_tree = new_tree;

        self.last_source_code = new_source_code;
    }

    pub fn print_recursive(&self)
    {
        match &self.last_tree
        {
            Some(tree) => { Diff::recursive_descent(&tree.root_node(), String::new()); },
            None => println!("None")
        }
        
    }

    fn recursive_descent(node : &tree_sitter::Node, mut depth_string : String)
    {
        //pretty print the node with the tabs in front
        println!("{}{:#?}",depth_string, node);

        //add another space for the depth
        depth_string.push(' ');

        for n in node.children()
        {
            Diff::recursive_descent(&n, depth_string.clone());
        }
    }
}

/*
#[wasm_bindgen]
pub struct Pipetext
{
    /// The contents of the buffer.
    text: Rope,
    /// The CRDT engine, which tracks edit history and manages concurrent edits.
    engine: Engine

}

#[wasm_bindgen]
impl Pipetext
{
    
    pub fn from_string(source_code : &str) -> Pipetext
    {
        Pipetext
        {
            //diff : Diff::from_string(source_code, Parsable::Javascript),
            text : Rope::from_str("").unwrap(),
            engine : Engine::new(Rope::from_str(source_code).unwrap())
        }
    }

    pub fn get_doc_string(&self) -> String { String::from(self.text.clone()) }

    //let iv = Interval::new(region.min(), region.max());
    //<T: Into<Rope>>
    pub fn insert(&mut self, start : usize, end : usize, text: &str) 
    {
        let iv = Interval::new(start, end);
        let rope : Rope = text.into();
        let mut builder = DeltaBuilder::new(self.text.len());
        builder.replace(iv, rope.clone());
        self.add_delta(builder.build());
    }

    /// Deletes the given regions.
    pub fn delete(&mut self, start : usize, end : usize) 
    {
        let iv = Interval::new(start, end);
        let mut builder = DeltaBuilder::new(self.text.len());
        if !iv.is_empty() { builder.delete(iv); }
        if !builder.is_empty() { self.add_delta(builder.build()); }
    }

    fn add_delta(&mut self, delta: RopeDelta) 
    {
        let head_rev_id = self.engine.get_head_rev_id();
        let undo_group = 0;//self.calculate_undo_group();
        //self.last_edit_type = self.this_edit_type;
        let priority = 0x10000;
        self.engine.edit_rev(priority, undo_group, head_rev_id.token(), delta);
        self.text = self.engine.get_head().clone();
    }

    /*
    pub fn calculate_undo_group(&mut self) -> usize 
    {
        let has_undos = !self.live_undos.is_empty();
        let force_undo_group = self.force_undo_group;
        let is_unbroken_group = !self.this_edit_type.breaks_undo_group(self.last_edit_type);

        if has_undos && (force_undo_group || is_unbroken_group) {
            *self.live_undos.last().unwrap()
        } else {
            let undo_group = self.undo_group_id;
            self.gc_undos.extend(&self.live_undos[self.cur_undo..]);
            self.live_undos.truncate(self.cur_undo);
            self.live_undos.push(undo_group);
            if self.live_undos.len() <= MAX_UNDOS {
                self.cur_undo += 1;
            } else {
                self.gc_undos.insert(self.live_undos.remove(0));
            }
            self.undo_group_id += 1;
            undo_group
        }
    }*/

}*/