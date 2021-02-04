use tree_sitter::{ InputEdit, Point };
use std::cmp;

use regex::Regex;

use unicode_segmentation::UnicodeSegmentation;

//Gets the start and end of the edited region
fn getDiff(old_text_unicode : &Vec<(usize, &str)>, new_text_unicode : &Vec<(usize, &str)>, cursorIndex: usize) -> (usize, usize, isize)
{
    let delta : isize = (new_text_unicode.len() as isize) - (old_text_unicode.len() as isize);

    let limit : usize = cmp::max(0, (cursorIndex as isize - delta)as usize);
    let mut end = if old_text_unicode.len() == 0 {0} else { old_text_unicode.len() - 1 };

    while end > limit && old_text_unicode[end - 1].1 == new_text_unicode[((end as isize + delta) - 1) as usize].1
    {
        end -= 1;
    }
    let mut start : usize = 0;
    let startLimit : isize = cursorIndex as isize - cmp::max(0, delta);
    while (start as isize) < startLimit && old_text_unicode[start].1 == new_text_unicode[start].1 {
        start += 1;
    }

    println!("start : {}, end : {}, delta : {}", start, end, delta);

    return (start, end, delta-1);
}

//takes a string and the change indices and converts them to tree sitter Point objects 
fn find_points(old_text : &str, new_text : &str, start_byte : usize, end_byte : usize, delta_byte : usize) -> (Point, Point, Point)
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\r\n|\r|\n").unwrap();
    }
    
    //capture all the end indices of the newlines
    let mut new_line_byte_indices : Vec<usize> = RE.find_iter(old_text)
    .map(|nl_match|
    {
        nl_match.end()
    }).collect();

    //push the length of the string to account for the last row
    new_line_byte_indices.push(old_text.len());

    //if we don't set anything 
    //its either a one line string or the indices are just inside the first line
    let mut start_line = 0;
    let mut end_line = 0;

    //the last i+1 check will be for the length of the string so i covers
    //all the indices collected in the map function above
    for i in 0..new_line_byte_indices.len() - 1
    {
        //start_line = i + 1 because we don't check the 0th line
        if new_line_byte_indices[i] < start_byte && start_byte < new_line_byte_indices[i+1] { start_line = i + 1; }
        if new_line_byte_indices[i] < end_byte && end_byte < new_line_byte_indices[i+1] { end_line = i + 1; }
    }

    //find the start column by finding 
    //the difference between the start_index and the start_line start
    let start_column = match start_line == 0 
    { 
        true => start_byte,
        false => start_byte - (new_line_byte_indices[start_line - 1]-1)
    };

    //find the end column by finding 
    //the difference between the end_index and the end_line start
    let end_column = match end_line == 0 
    { 
        true => end_byte,
        false => end_byte - (new_line_byte_indices[end_line - 1]-1)
    };

    //find the newlines in the slice of new text
    let delta_new_line_indices : Vec<usize> = RE.find_iter(&new_text[start_byte..(end_byte + delta_byte)])
    .map(|nl_match|
    {
        nl_match.end()
    }).collect();

    //The absolute delta line index
    //i.e. relative to the entire newText 
    //this is the index where the line, on shich the delta ends, starts

    //the line in which the delta ends
    let (delta_line, absolute_delta_line_index) = match delta_new_line_indices.len()
    {
        0 => { (end_line, end_line) }
        _ => { (end_line + (delta_new_line_indices.len()-1), end_byte + delta_new_line_indices[end_line]) }
    };

    let delta_column = match delta_line == 0
    {
        //if we're on the same line then 
        //we simply add the delta to our current column
        true =>  end_column + delta_byte,

        //otherwise we're on a new line so we find the difference between the end index 
        //and the lines beginning 
        false => (delta_byte - absolute_delta_line_index)
    };

    (Point::new(start_line, start_column), Point::new(end_line, end_column), Point::new(delta_line, delta_column))
}

pub fn get_input_edit_from_diff(old_source_code : &str, new_source_code : &str, cursorIndex : usize) -> InputEdit
{
    //create the initial unicode character iterators and clone them to access indices below
    //TODO: maybe map these to &char for faster comparison?
    let old_text_unicode : Vec<(usize, &str)> = UnicodeSegmentation::grapheme_indices(old_source_code, true).collect();
    let new_text_unicode : Vec<(usize, &str)> = UnicodeSegmentation::grapheme_indices(new_source_code, true).collect();

    let (start, end, delta) = getDiff(&old_text_unicode, &new_text_unicode, cursorIndex);

    let start_byte = if old_text_unicode.len() > 0 {old_text_unicode[start].0} else { 0 };
    let end_byte = if old_text_unicode.len() > 0 {old_text_unicode[end].0} else { 0 };
    let delta_byte = new_text_unicode[(end as isize + delta) as usize].0;

    let (start_point, end_point, delta_point) = find_points(old_source_code, new_source_code, start_byte, end_byte, delta_byte);

    let edit = InputEdit 
    {
        start_byte: start_byte,
        old_end_byte: end_byte + 1,
        new_end_byte: end_byte + delta_byte,
        start_position: start_point,
        old_end_position: end_point,
        new_end_position: delta_point,
    };

    println!("{:#?}",edit);

    edit
}