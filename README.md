# Command Line Todo app
It allows the user to add a todo, mark a todo as done and do subsequence search for todos
based on tags and descriptions.

## Examples
```add "buy bread" #groceries #bread```, adds a todo with description buy bread and tag groceries and bread

Every add query returns the id of the todo, so that has to be used to mark it as done.
For example, if the above query returned 4, then
```done 4``` will mark that todo as completed.

```search word1 word2 #tag1 #tag2``` will return all the todos for which every word should 
subsequence match with atleast one word in the description and every tag should match at least
1 tag of the todo.

Realtime example:

### Sample input
10
add "buy bread" #groceries
add "buy milk" #groceries
add "call parents" #relatives
search #groceries
search buy
search a
done 0
search a
done 2
search a

### Sample output
0
1
2
2 item(s) found
1 "buy milk" #groceries
0 "buy bread" #groceries
2 item(s) found
1 "buy milk" #groceries
0 "buy bread" #groceries
2 item(s) found
2 "call parents" #relatives
0 "buy bread" #groceries
done
1 item(s) found
2 "call parents" #relatives
done
0 item(s) found
