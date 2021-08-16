### Multithreading
This is a concept for parallel programming and basic architecture to derive the system resource in multiple sequence as context and execute the command, 

**Example**
A system with single core execute multiple programs based by dividing the clock cycle as schedule, it woudl check the program1 and move to programm2 .. and so on in round robin fashio

__Programming__
To implement the multithreading in the *Rust* we create a module with the name **"threading_example"** where multiple fn execute in parallel and take processing, 

this will raise the race condition (same variable/resource) been consumed (conflict) with multiple user, we avoid the condition by simple print statement with a wait (sleep) to demostrate the delay each execution take

>> This illustrate the multiple car function execute in parallel to make the car drivable

### Async / Await
Async is also a parallel programming concept but the concept is to avoid the complexities and focus on the delivables where we have to perform the task in sequence and wait only for those command that relay on other programm/resource to be performed

Aysnc is come from Asynchronous which means the clock cycle are not same, that means the communication been performed with different context there used to be a medium that help to make the tansition in sync

**Example**
You need to make a load in an online bank, the bank will review the profile by asking a third party credit rating to evaluate the risk, this call will be async as the system not depend on banking application, it would be an async call, (second the database store where database is indepedend from main application)

__Programming__
In the programming we define the **Async_example** module, this would illustrate the differnt part of the digital car function, in order to make sequence, 
1. The transmission would not been trigger until the engine is on
2. The AutoMode will not be execute until the ECU starts

As these depdendencies need to be meet for each execution, they would run the program in task focus and only wait when the execution of the prior task been completed

>> This illustrate the multiple core function that depend on the correct execution on the main function will be executed first (priority and task focus)