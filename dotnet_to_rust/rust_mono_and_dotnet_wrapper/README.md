# Calling DLLs directly from rust


via mono:

Here we used the wrapper_mono package. This did not work, since it returns no error, when loading some class, but actually returns nothing for whatever reason


via dotnetcorehost:

Probably a similar problem. It seems like we cannot find the function we want to execute  