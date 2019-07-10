use super:: IncrementalDiff::IncrementalDiff;
use super::lang::Parsable;

#[test]
fn test_add() 
{
    let mut differ : IncrementalDiff = IncrementalDiff::new();
    differ.setLanguage(Parsable::Javascript);

    differ.parse("function bubble_Sort(a)
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
    }");
   
}