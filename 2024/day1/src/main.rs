use std::io::{BufReader, BufRead};
use std::fs::File;
use std::iter::zip;

fn main() {

    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let mut left: Vec<u64> = Vec::new();
    let mut right: Vec<u64> = Vec::new();

    /* Parse input into two lists of u64. */
    for line in lines
    {
        let mut lineiter = line.split_whitespace();
        left.push(lineiter.next().unwrap().parse::<u64>().unwrap());
        right.push(lineiter.next().unwrap().parse::<u64>().unwrap());
        assert_eq!(None, lineiter.next());
    }


    /* Sort lists */
    left.sort_unstable();
    right.sort_unstable();

    /* Iterate through and track total distance. */
    let mut total_dist: u64 = 0;

    let both = zip(left.iter(), right.iter());

    for (leftval, rightval) in both
    {
        if leftval > rightval
        {
            total_dist += leftval - rightval;
        }
        else
        {
            total_dist += rightval - leftval;
        }
    }

    println!("[Part 1] Total distance is {}.", total_dist);

    /* Calculate similatity score from ordered lists. */
    let mut similarity_score: u64 = 0;

    let mut leftiter = left.iter();
    let mut rightiter = right.iter();

    /* Track rightnext to not miss first occurence after switching left value. */
    let mut rightnext: Option<&u64> = None;
    /* Tack last value and occurence to handle multiple occurences. */
    let mut lastleft: u64 = 0;
    let mut lastocc: u64 = 0;

    loop
    {
        let leftnext = leftiter.next();
        if None == leftnext {
            break;
        }
        let leftval = leftnext.unwrap();

        /* Check if this leftval was already handled. */
        if lastleft == *leftval{
            similarity_score += lastocc * lastleft;
            continue;
        }

        let mut occurence_right: u64 = 0;

        /* If rightnext holds some number, perform checks. */
        if None != rightnext
        {
            let rightval = rightnext.unwrap();
            
            /* If rightval is greater leftval, we can skip leftval. */
            if rightval > leftval {
                continue;
            }

            /* If rightval equals leftval, already increase occurence and keep oin checking. */
            if rightval == leftval{
                occurence_right = 1;
            }
        }

        loop
        {
            rightnext = rightiter.next();
            if None == rightnext {
                break;
            }
            let rightval = rightnext.unwrap();

            /* smaller value can be disregarded. */
            if rightval < leftval{
                continue;
            }
            /* bigger value means we have to advance the left value. */                
            else if rightval > leftval{
                break;
            }
            else {
                /* Same value -> increase occurence. */
                occurence_right += 1;
            }
        }

        /* Increase similarity score after every occurence on the right was found. */
        similarity_score += occurence_right * leftval;
        lastleft = *leftval;
        lastocc = occurence_right;
    }

    println!("[Part 2] Similarity score is {}.", similarity_score);

}
