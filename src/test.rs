use super::Diff;
use super::lang::Parsable;

#[test]
fn test_add() 
{
    let mut javascript_differ : Diff = Diff::new(Parsable::Javascript);

    let test_str = "function bubble_Sort(a)
    {
    var swapp;
    var n = a.length-1;
    var x=a;
    do {
        swapp = false;
        for (var i=0; i < n; i++)
        {
            if (x[i] < x[i+1])
            {
               var temp = x[i];
               x[i] = x[i+1];
               x[i+1] = temp;
               swapp = true;
            }
        }
        n--;
    } while (swapp);
    return x; 
    }";

    let cursor_index = test_str.len();

    //println!("Len : {}", cursor_index);

    javascript_differ.update(test_str, cursor_index);

    //javascript_differ.print_recursive();
   
}

#[test]
fn test_update()
{
    let mut javascript_differ : Diff = Diff::new(Parsable::Javascript);

    let first_string = "let x = 1; console.log(x);";

    let cursor_index = first_string.len()-1;

    //initial string parse
    javascript_differ.update(first_string, cursor_index);

    //incremenetal parse the second string
    let second_string = "const x = 1; console.log(x);";

    //initial string parse
    javascript_differ.update(second_string, 4);

    javascript_differ.print_recursive();

}