# A program to store email addresses

## Main loop

```c
init_list();
do {
    show_menu();
    get_user_choice();
    execute_choice();
} while(!quit());
```

## Menu options

- `[x]` Set new entry
  - Find a free `struct`
  - Ask the user to fill each entry
    - Show petition message
    - Get the user input
    - Check the length of the input
- `[x]` Delete existing entry
- `[x]` Show list
- `[x]` Search entry
- `[x]` Save list in file
- `[x]` Read list from file
