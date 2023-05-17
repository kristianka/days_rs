# days_rs

Course project for Ohjelmoinnin syventävät teknologiat -course. This is a Rust conversion of https://github.com/kristianka/days_cpp which is based from https://github.com/jerekapyaho/days_cpp.
Please read the readme from there to know how to use the program!

## Building and running the program 

### For all platforms:

- Make sure you have Rust tools installed like Rustup and Cargo

- Clone the repository

### On Linux:

- Build the program with ```cargo build --release```

- Run the program, the ```days_rs``` is located in ```target/release``` folder. For example on Ubuntu ```./days_rs list```  will list all the events.

### On Windows:

- Build the program with ```cargo build --release```

- Run the program, the ```days_rs.exe``` is located in ```target/release``` folder. For example on PowerShell ```.\days_rs.exe list```  will list all the events.

---

### Example ```events.csv``` file

```
date,category,description
1985-12-31,computing,C++ released
2010-01-01,sport,Go
2014-11-12,computing,.NET Core released
2020-12-15,computing,C++20 released
2022-09-20,computing,Java SE 19 released
2023-01-10,computing,Rust 1.66.1 released
2023-03-12,games,New game releases
2023-05-09,school,Today is Tuesday
2023-05-10,school,Today is Wednesday
2023-05-10,school,Starting course work
2023-05-10,,This is a event without category
2023-06-23,holidays,Juhannusaatto
2023-06-30,quarters,Quarter 2 ends
2023-12-31,computing,C++23 released
2030-01-01,,No category
2030-01-01,,No category
2038-01-19,computing,Unix clock rolls over
2023-05-15,school,Days_rs finished
```
