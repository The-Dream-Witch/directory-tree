<h6>
Mallory S. Hawke</br>
CS410P, Spring 2021</br>
Homework 3
</h6>
<div align = "center">
<h1>Directory Tree Simulator</h1>
</div>


<h3>What does it do?</h3>
This is a library that implements a simulated portion of an operating system's directory
system. It allows for instatiation of an Operating System State (OsState), 
Directory Tree (DTree), or Directory Entry (DEnt), which enables the construction,
and navigation, of directory 'paths.' 

<h3>How does it do it?</h3>

It builds off of a (completely unalterable, due to the assignment constraints)
skeleton, written by Bart Massey, which consisted of the following pieces:

* enum: DirError
* type: Result
* structs: DTree, DEnt, OsState

Although the basic framework, and many function / method headers, were provided, it was up
to me to implement the core features:

- DEnt objects have the ability to be created indepent of any other structure, as well as to return a comprehensive list of all directories immediate to itself (its subdirectory).

- DTrees have the ability to be created independent of an OsState, can create a new directory (DEnt) of a given name as a child, can return a comprehensive listing of all directories, in order, as they appear within the tree. Additionally, it has two pair (one for mutable purposes, one for immutable) of functions (wrapper function / body function) which allow for traversal through the tree, and execution of a closure once having navigated to the appropriate directory.

- OsState makes use of the groundwork laid by the previous two implementations, and builds upon it. As with the previous two, an OsState may be instantiated, a new directory may be created within it, and it can return a vector containing all paths, as they appear in the tree; unlike the other two implementations, OsState needs to handle keeping track of the current working directory, and has its own method for doing so. This method ultimately utilizes DEnt's subdirectory navigation functionality as a way to check that the directory being changed to is valid.

For clarity, each OsState has a DTree and a current working directory (cwd), each DTree has children (a DEnt vector), and each DEnt has a subdirectory (a DTree vector) as well as a name.

<h3>How do we know it works/doesn't break?</h3>
Without some form of formal verification, we don't. But the following tests were written with the intent of making it pretty likely that the library functions as intended:
</br>
</br>

<h4>OsState Tests</h4>

* **osstate_rand** - Creates a new OsState, then randomly generates ten, ten character long, strings which it stores in a vector. Each string in the vector is then fed into the OsState as a new directory, the OsState's current working directory is changed to the newly created one, and the string is concatenated onto a path string. At the end, the current working directory is set to root, and the path string is compared to the result of OsState's paths method.</br>
* **osstate_bad_chdir** - Creates a new OsState, creates a new directory `named: x`, then tries to change the current working directory to a directory `named: q` , which does not exist. Expected to panic.</br>
* **osstate_bad_mkdir** - Attempts the creation of a new directory `named: /`; expected to panic.</br>
* **osstate_double** - Attempts the creation of two directories, back to back, both `named: a`. Also expected to panic.</br></br>

<h4>DTree Tests</h4>

* **dtree_static** - Attempts to create a series of directories with predetermined names, in a predetermined order; once the directories have been created, the result of dtree's `paths` method is compared against a predetermined array.</br>
* **dtree_rand** - Randomly generates a ten character long string, then uses that string to create a new directory. Compares the results of the `paths` method to an array containing the generated path. </br>
* **dtree_slash** - Creates a new DTree, then attempts to make a directory `named: /a`; expected to panic. </br>
* **dtree_double** - Creates a new DTree, then attempts to create two directories, back to back, both `named: a`; expected to panic.