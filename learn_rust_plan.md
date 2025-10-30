# **14-Day Rust Learning Plan (1.5 Hours/Day)**

This intensive 2-week plan is designed to take you from zero to understanding Rust's core concepts. The key is consistency and *embracing* the compiler's errors—they are your teacher. You will use GitHub to track your progress and build a portfolio.  
**Core Resources:**

1. **The Rust Programming Language ("The Book"):** [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/) (This will be your primary guide)  
2. **Your Code Editor:** VS Code with the rust-analyzer extension is highly recommended.  
3. **The Rust Compiler (rustc):** Listen to its errors and warnings.  
4. **A GitHub Account:** You will commit and push your code daily.

## **Week 1: The Fundamentals & The Core Concept**

### **Day 1: Setup & "Hello, World\!"**

* **Goal:** Get your environment running and understand the basics of a Rust project.  
* **Reading (40 min):** "The Book" \- Chapter 1: Getting Started.  
* **Activity (50 min):**  
  1. Install rustup (which includes rustc and cargo).  
  2. Set up your code editor with the rust-analyzer extension.  
  3. Create a new project with cargo new hello\_cargo.  
  4. **GitHub:**  
     * cd hello\_cargo and run git init.  
     * Create a new public repository on GitHub named rust-learning-journey.  
     * Add the remote (git remote add origin ...) and push your initial project (git push \-u origin main).  
  5. Run cargo build and cargo run. Get comfortable with the Cargo workflow.

### **Day 2: Guessing Game & Common Concepts (Part 1\)**

* **Goal:** Write your first real program and learn basic syntax.  
* **Reading (40 min):** "The Book" \- Chapter 2 (Programming a Guessing Game) & Chapter 3.1-3.2 (Variables, Data Types).  
* **Activity (50 min):**  
  1. Create a new project cargo new guessing\_game.  
  2. **Type out the Guessing Game from Chapter 2\.** Don't just copy-paste. Pay attention to let mut, match, and Result.  
  3. Write small programs *in your hello\_cargo project's main.rs* to experiment with variables, mutability (mut), and shadowing.  
  4. **GitHub:** Commit and push your work. (e.g., git add ., git commit \-m "Day 2: Guessing Game and variables", git push).

### **Day 3: Common Concepts (Part 2\)**

* **Goal:** Learn functions, control flow, and compound data types.  
* **Reading (40 min):** "The Book" \- Chapter 3.3-3.5 (Functions, Comments, Control Flow).  
* **Activity (50 min):**  
  1. Work inside your hello\_cargo project.  
  2. Write a function that takes two numbers and returns their sum.  
  3. Create a tuple and an array; access their elements.  
  4. Write a for loop that prints numbers 1 through 10\.  
  5. **Challenge:** Write a program to calculate the Nth Fibonacci number (using a loop).  
  6. **GitHub:** Commit and push your work. (e.g., git commit \-m "Day 3: Functions and control flow")

### **Day 4: Understanding Ownership (Part 1: The Theory)**

* **Goal:** *This is the most important day.* Understand the core concept of Ownership.  
* **Reading (60 min):** "The Book" \- Chapter 4.1 (What is Ownership?). Read this section *twice*. Pay attention to Stack vs. Heap, Move, Clone, and Copy.  
* **Activity (30 min):**  
  1. Work inside your hello\_cargo project.  
  2. Create a String. Assign it to another variable. Try to use the *first* variable. See the "move" error. *Comment out the broken code.*  
  3. "Fix" the error using .clone() and write a comment explaining why.  
  4. Do the same with an i32. Notice it works without clone(). Understand *why* (Copy trait).  
  5. **GitHub:** Commit and push your experiments. (e.g., git commit \-m "Day 4: Ownership experiments")

### **Day 5: Ownership (Part 2: References & Borrowing)**

* **Goal:** Learn how to access data without taking ownership.  
* **Reading (40 min):** "The Book" \- Chapter 4.2 (References & Borrowing).  
* **Activity (50 min):**  
  1. Work inside your hello\_cargo project.  
  2. Write a function calculate\_length that takes a *reference* (\&String) and returns its length.  
  3. Write a function change that takes a *mutable reference* (\&mut String) and appends text to it.  
  4. Deliberately try to create two mutable references in the same scope. See the compiler error. *Comment out the broken code* and explain the error.  
  5. **GitHub:** Commit and push your work. (e.g., git commit \-m "Day 5: References and borrowing")

### **Day 6: Ownership (Part 3: Slices)**

* **Goal:** Understand how to reference parts of a collection.  
* **Reading (30 min):** "The Book" \- Chapter 4.3 (The Slice Type).  
* **Activity (60 min):**  
  1. Work inside your hello\_cargo project.  
  2. Write a function first\_word that takes a String (or \&String) and returns a \&str (a string slice).  
  3. Test it by passing in slices of a String (e.g., \&my\_string\[..5\]).  
  4. Apply this concept to an array of integers and write a function that takes a slice &\[i32\].  
  5. **GitHub:** Commit and push your work. (e.g., git commit \-m "Day 6: Slices")

### **Day 7: Structs**

* **Goal:** Learn how to create custom data types.  
* **Reading (40 min):** "The Book" \- Chapter 5.1-5.2 (Defining Structs, Example Program).  
* **Activity (50 min):**  
  1. Work inside your hello\_cargo project.  
  2. Define a User struct with username, email, and active fields. Create an instance.  
  3. Define a Rectangle struct with width and height.  
  4. Write a function area that takes a reference to a Rectangle (\&Rectangle) and returns its area.  
  5. **GitHub:** Commit and push your work. (e.g., git commit \-m "Day 7: Structs")

## **Week 2: "Rust-y" Structures & Real-World Code**

### **Day 8: Struct Methods**

* **Goal:** Attach behavior to your custom data types.  
* **Reading (30 min):** "The Book" \- Chapter 5.3 (Method Syntax).  
* **Activity (60 min):**  
  1. Work inside your hello\_cargo project.  
  2. Refactor your Rectangle code: Create an impl block.  
  3. Move the area function into the impl block to be a *method* (i.e., it takes \&self).  
  4. Add another method can\_hold(\&self, other: \&Rectangle) that checks if a second rectangle can fit inside the first.  
  5. Add an "associated function" (like new() or square()) that acts as a constructor.  
  6. **GitHub:** Commit and push your work. (e.g., git commit \-m "Day 8: Struct methods")

### **Day 9: Enums & match**

* **Goal:** Learn Rust's powerful enum system for modeling state.  
* **Reading (40 min):** "The Book" \- Chapter 6.1-6.2 (Enums, match).  
* **Activity (50 min):**  
  1. Work inside your hello\_cargo project.  
  2. Define an enum called Message with variants like Quit, Move { x: i32, y: i32 }, Write(String), and ChangeColor(u8, u8, u8).  
  3. Write a function that takes a Message and matches on its variants, printing a different output for each.  
  4. Define a Coin enum and a function that uses match to return its value in cents.  
  5. **GitHub:** Commit and push your work. (e.g., git commit \-m "Day 9: Enums and match")

### **Day 10: Option & if let**

* **Goal:** Understand Rust's primary way of handling nullability.  
* **Reading (30 min):** "The Book" \- Chapter 6.3 (if let). Reread the Option\<T\> part of Chapter 6.1.  
* **Activity (60 min):**  
  1. Work inside your hello\_cargo project.  
  2. Write a function that divides two f64 numbers, but returns Option\<f64\>. Return None if the denominator is zero.  
  3. Call this function and use match to handle the Some(value) and None cases.  
  4. Rewrite the Coin example (or another simple match) to use if let for a specific case.  
  5. **GitHub:** Commit and push your work. (e.g., git commit \-m "Day 10: Option and if let")

### **Day 11: Collections (Vec & String)**

* **Goal:** Learn the most common collections.  
* **Reading (40 min):** "The Book" \- Chapter 8.1-8.2 (Vectors, Strings).  
* **Activity (50 min):**  
  1. Work inside your hello\_cargo project.  
  2. Create a Vec\<i32\>. Add numbers to it. Read a value by index.  
  3. Use .get() to safely access an index, and handle the Option it returns.  
  4. Iterate over a vector with a for loop (immutably) and mutably.  
  5. Concatenate two Strings. Iterate over a String's characters.  
  6. **GitHub:** Commit and push your work. (e.g., git commit \-m "Day 11: Vectors and Strings")

### **Day 12: Collections (HashMap) & Error Handling (Part 1\)**

* **Goal:** Learn key-value storage and introduce Result.  
* **Reading (45 min):** "The Book" \- Chapter 8.3 (HashMaps) & 9.1-9.2 (panic\!, Result).  
* **Activity (45 min):**  
  1. Work inside your hello\_cargo project.  
  2. Create a HashMap to store team names (key) and scores (value).  
  3. Insert, retrieve, and iterate over key-value pairs.  
  4. Use the entry API to update a score or insert if it doesn't exist.  
  5. **GitHub:** Commit and push your work. (e.g., git commit \-m "Day 12: HashMaps and Result")

### **Day 13: Error Handling (Part 2: The ? Operator)**

* **Goal:** Master clean, idiomatic error handling.  
* **Reading (30 min):** "The Book" \- Chapter 9.2 (Propagating Errors).  
* **Activity (60 min):**  
  1. Work inside your hello\_cargo project.  
  2. Write a function that reads from a file (e.g., std::fs::read\_to\_string). This function returns a Result.  
  3. Handle the Result using a match statement.  
  4. Refactor the function to use the ? operator to propagate the error. This is a *critical* idiom.  
  5. **GitHub:** Commit and push your work. (e.g., git commit \-m "Day 13: Error handling with ?")

### **Day 14: Mini-Project: Command Line To-Do List**

* **Goal:** Synthesize everything you've learned.  
* **Reading (10 min):** "The Book" \- Chapter 12 (An I/O Project) \- Skim this to see how it's structured.  
* **Activity (80 min):**  
  1. **Plan:** Create a *new project* cargo new todo\_cli and its own GitHub repository.  
  2. **Functionality:**  
     * cargo run \-- add "Buy milk"  
     * cargo run \-- list  
     * cargo run \-- complete 1 (to complete item 1\)  
  3. **Tools to use:**  
     * std::env::args() to get command-line arguments.  
     * struct Task { name: String, completed: bool }  
     * Vec\<Task\> to hold the list.  
     * std::fs::write and std::fs::read\_to\_string to save/load the list (use JSON with serde if you feel ambitious, or just a simple text format).  
     * Result and ? for all file I/O.  
     * match or if let to parse the arguments.  
  4. **GitHub:** Commit and push your mini-project to its new repository.

## **Tips for Success**

* **Code Every Day:** 90 minutes of focused effort is better than 6 hours on a weekend.  
* **Read the Errors:** The Rust compiler is your best friend. It's strict, but it's also *extremely* helpful. Read its messages fully.  
* **Don't "Fight" the Borrower Checker:** When you get an ownership error, don't try to find a "quick fix." Stop and ask: "Who owns this value? Who is borrowing it? Is it mutable? What is its lifetime?" Answering these questions is how you learn.  
* **Be Patient:** Days 4-6 (Ownership) will be the hardest. It's normal to feel confused. Keep pushing, and it will "click."

## **Beyond the 2 Weeks: The Flagstone Project**

Congratulations on finishing the 14-day sprint\! You now have a solid foundation. The next step is to solidify this knowledge by building something larger. This project is self-directed and will take longer than 90 minutes per day.  
Choose **one** of the following projects, create a new cargo project and GitHub repository for it, and start building.

### **Project Idea 1: Multi-threaded Web Server**

* **Description:** Build a web server that listens for TCP connections and serves a simple HTML file or echoes the request.  
* **Key Concepts:** std::net (TCP listeners), std::thread (to handle multiple connections), String parsing (for HTTP requests), Error Handling (Result), Ownership (moving data to threads).  
* **Reference:** "The Book" \- Chapter 20 (This is an advanced project but directly covered in the book).

### **Project Idea 2: Personal Budget Tracker (CLI)**

* **Description:** A more advanced CLI tool than the to-do list.  
* **Functionality:**  
  * budget add \--type income \--amount 500 \--desc "Paycheck"  
  * budget add \--type expense \--amount 40 \--desc "Groceries" \--category "Food"  
  * budget list \--category "Food"  
  * budget report (show total income, total expenses, and net balance).  
* **Key Concepts:** Advanced argument parsing (using a crate like clap), Structs (Transaction), Enums (TransactionType), HashMap (to group by category), File I/O (using serde for JSON).

### **Project Idea 3: Text-Based Adventure Game**

* **Description:** A simple game where the user can move between rooms and interact with items.  
* **Functionality:**  
  * User is in a Room which has a description and exits.  
  * User can type commands like go north, look, take key.  
  * The world is defined by connected Room structs.  
* **Key Concepts:** Structs (Room, Player, Item), HashMap (to store the game world, e.g., (x, y) \-\> Room), String parsing (for commands), Option (for items that may or may not be present), match.